macro_rules! deps {
    () => {
        Status!();
        Note!();
        Clone!();
        Entry!();
    };
}

macro_rules! RewriteSource {
    () => {
        deps!();
        # [doc = " Either an index entry for renames or another directory entry in case of copies."] # [derive (Clone , PartialEq , Debug)] pub enum RewriteSource { # [doc = " The source originates in the index and is detected as missing in the working tree."] # [doc = " This can also happen for copies."] RewriteFromIndex { # [doc = " The entry that is the source of the rewrite, which means it was removed on disk,"] # [doc = " equivalent to [Change::Removed](gix_status::index_as_worktree::Change::Removed)."] # [doc = ""] # [doc = " Note that the [entry-id](gix_index::Entry::id) is the content-id of the source of the rewrite."] source_entry : gix_index :: Entry , # [doc = " The index of the `source_entry` for lookup in [`gix_index::State::entries()`] - useful to look at neighbors."] source_entry_index : usize , # [doc = " The repository-relative path of the `source_entry`."] source_rela_path : BString , # [doc = " The computed status of the `source_entry`."] source_status : gix_status :: index_as_worktree :: EntryStatus < () , crate :: submodule :: Status > , } , # [doc = " This source originates in the directory tree and is always the source of copies."] CopyFromDirectoryEntry { # [doc = " The source of the copy operation, which is also an entry of the directory walk."] # [doc = ""] # [doc = " Note that its [`rela_path`](gix_dir::EntryRef::rela_path) is the source of the rewrite."] source_dirwalk_entry : gix_dir :: Entry , # [doc = " `collapsed_directory_status` is `Some(dir_status)` if this `source_dirwalk_entry` was part of a directory with the given"] # [doc = " `dir_status` that wasn't the same as the one of `source_dirwalk_entry` and"] # [doc = " if [gix_dir::walk::Options::emit_collapsed] was [CollapsedEntriesEmissionMode::OnStatusMismatch](gix_dir::walk::CollapsedEntriesEmissionMode::OnStatusMismatch)."] # [doc = " It will also be `Some(dir_status)` if that option was [CollapsedEntriesEmissionMode::All](gix_dir::walk::CollapsedEntriesEmissionMode::All)."] source_dirwalk_entry_collapsed_directory_status : Option < gix_dir :: entry :: Status > , # [doc = " The object id as it would appear if the entry was written to the object database."] # [doc = " It's the same as [`dirwalk_entry_id`](Item::Rewrite), or `diff` is `Some(_)` to indicate that the copy"] # [doc = " was determined by similarity, not by content equality."] source_dirwalk_entry_id : gix_hash :: ObjectId , } , }
    };
}

RewriteSource!()