use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Vfs {
    /// Id of the given path if it exists in the `Vfs` and is not deleted.
    pub fn file_id(&self, path: &VfsPath) -> Option<(FileId, FileExcluded)> {
        let file_id = self.interner.get(path)?;
        let file_state = self.get(file_id);
        match file_state {
            FileState::Exists(_) => Some((file_id, FileExcluded::No)),
            FileState::Deleted => None,
            FileState::Excluded => Some((file_id, FileExcluded::Yes)),
        }
    }
    /// File path corresponding to the given `file_id`.
    ///
    /// # Panics
    ///
    /// Panics if the id is not present in the `Vfs`.
    pub fn file_path(&self, file_id: FileId) -> &VfsPath {
        self.interner.lookup(file_id)
    }
    /// Returns an iterator over the stored ids and their corresponding paths.
    ///
    /// This will skip deleted files.
    pub fn iter(&self) -> impl Iterator<Item = (FileId, &VfsPath)> + '_ {
        (0..self.data.len())
            .map(|it| FileId(it as u32))
            .filter(move |&file_id| matches!(self.get(file_id), FileState::Exists(_)))
            .map(move |file_id| {
                let path = self.interner.lookup(file_id);
                (file_id, path)
            })
    }
    /// Update the `path` with the given `contents`. `None` means the file was deleted.
    ///
    /// Returns `true` if the file was modified, and saves the [change](ChangedFile).
    ///
    /// If the path does not currently exists in the `Vfs`, allocates a new
    /// [`FileId`] for it.
    pub fn set_file_contents(&mut self, path: VfsPath, contents: Option<Vec<u8>>) -> bool {
        let _p = span!(Level::INFO, "Vfs::set_file_contents").entered();
        let file_id = self.alloc_file_id(path);
        let state: FileState = self.get(file_id);
        let change = match (state, contents) {
            (FileState::Deleted, None) => return false,
            (FileState::Deleted, Some(v)) => {
                let hash = hash_once::<FxHasher>(&*v);
                Change::Create(v, hash)
            }
            (FileState::Exists(_), None) => Change::Delete,
            (FileState::Exists(hash), Some(v)) => {
                let new_hash = hash_once::<FxHasher>(&*v);
                if new_hash == hash {
                    return false;
                }
                Change::Modify(v, new_hash)
            }
            (FileState::Excluded, _) => return false,
        };
        let mut set_data = |change_kind| {
            self.data[file_id.0 as usize] = match change_kind {
                &Change::Create(_, hash) | &Change::Modify(_, hash) => FileState::Exists(hash),
                Change::Delete => FileState::Deleted,
            };
        };
        let changed_file = ChangedFile { file_id, change };
        match self.changes.entry(file_id) {
            Entry::Occupied(mut o) => {
                use Change::*;
                match (&mut o.get_mut().change, changed_file.change) {
                    (change, Delete) => *change = Delete,
                    (Create(prev, old_hash), Create(new, new_hash) | Modify(new, new_hash)) => {
                        *prev = new;
                        *old_hash = new_hash;
                    }
                    (Modify(prev, old_hash), Modify(new, new_hash)) => {
                        *prev = new;
                        *old_hash = new_hash;
                    }
                    (change @ Delete, Create(new, new_hash)) => {
                        *change = Modify(new, new_hash);
                    }
                    (change @ Delete, Modify(new, new_hash)) => {
                        stdx::never!();
                        *change = Create(new, new_hash);
                    }
                    (prev @ Modify(_, _), new @ Create(_, _)) => *prev = new,
                }
                set_data(&o.get().change);
            }
            Entry::Vacant(v) => set_data(&v.insert(changed_file).change),
        };
        true
    }
    /// Drain and returns all the changes in the `Vfs`.
    pub fn take_changes(&mut self) -> IndexMap<FileId, ChangedFile, BuildHasherDefault<FxHasher>> {
        mem::take(&mut self.changes)
    }
    /// Provides a panic-less way to verify file_id validity.
    pub fn exists(&self, file_id: FileId) -> bool {
        matches!(self.get(file_id), FileState::Exists(_))
    }
    /// Returns the id associated with `path`
    ///
    /// - If `path` does not exists in the `Vfs`, allocate a new id for it, associated with a
    ///   deleted file;
    /// - Else, returns `path`'s id.
    ///
    /// Does not record a change.
    fn alloc_file_id(&mut self, path: VfsPath) -> FileId {
        let file_id = self.interner.intern(path);
        let idx = file_id.0 as usize;
        let len = self.data.len().max(idx + 1);
        self.data.resize(len, FileState::Deleted);
        file_id
    }
    /// Returns the status of the file associated with the given `file_id`.
    ///
    /// # Panics
    ///
    /// Panics if no file is associated to that id.
    fn get(&self, file_id: FileId) -> FileState {
        self.data[file_id.0 as usize]
    }
    /// We cannot ignore excluded files, because this will lead to errors when the client
    /// requests semantic information for them, so we instead mark them specially.
    pub fn insert_excluded_file(&mut self, path: VfsPath) {
        let file_id = self.alloc_file_id(path);
        self.data[file_id.0 as usize] = FileState::Excluded;
    }
}
