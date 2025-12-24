use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Utf8DirEntry {
    fn new(inner: fs::DirEntry) -> io::Result<Self> {
        let path = inner
            .path()
            .try_into()
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?;
        Ok(Self { inner, path })
    }
    /// Returns the full path to the file that this entry represents.
    ///
    /// The full path is created by joining the original path to `read_dir`
    /// with the filename of this entry.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use camino::Utf8Path;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     for entry in Utf8Path::new(".").read_dir_utf8()? {
    ///         let dir = entry?;
    ///         println!("{}", dir.path());
    ///     }
    ///     Ok(())
    /// }
    /// ```
    ///
    /// This prints output like:
    ///
    /// ```text
    /// ./whatever.txt
    /// ./foo.html
    /// ./hello_world.rs
    /// ```
    ///
    /// The exact text, of course, depends on what files you have in `.`.
    #[inline]
    pub fn path(&self) -> &Utf8Path {
        &self.path
    }
    /// Returns the metadata for the file that this entry points at.
    ///
    /// This function will not traverse symlinks if this entry points at a symlink. To traverse
    /// symlinks use [`Utf8Path::metadata`] or [`fs::File::metadata`].
    ///
    /// # Platform-specific behavior
    ///
    /// On Windows this function is cheap to call (no extra system calls
    /// needed), but on Unix platforms this function is the equivalent of
    /// calling `symlink_metadata` on the path.
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8Path;
    ///
    /// if let Ok(entries) = Utf8Path::new(".").read_dir_utf8() {
    ///     for entry in entries {
    ///         if let Ok(entry) = entry {
    ///             // Here, `entry` is a `Utf8DirEntry`.
    ///             if let Ok(metadata) = entry.metadata() {
    ///                 // Now let's show our entry's permissions!
    ///                 println!("{}: {:?}", entry.path(), metadata.permissions());
    ///             } else {
    ///                 println!("Couldn't get metadata for {}", entry.path());
    ///             }
    ///         }
    ///     }
    /// }
    /// ```
    #[inline]
    pub fn metadata(&self) -> io::Result<Metadata> {
        self.inner.metadata()
    }
    /// Returns the file type for the file that this entry points at.
    ///
    /// This function will not traverse symlinks if this entry points at a
    /// symlink.
    ///
    /// # Platform-specific behavior
    ///
    /// On Windows and most Unix platforms this function is free (no extra
    /// system calls needed), but some Unix platforms may require the equivalent
    /// call to `symlink_metadata` to learn about the target file type.
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8Path;
    ///
    /// if let Ok(entries) = Utf8Path::new(".").read_dir_utf8() {
    ///     for entry in entries {
    ///         if let Ok(entry) = entry {
    ///             // Here, `entry` is a `DirEntry`.
    ///             if let Ok(file_type) = entry.file_type() {
    ///                 // Now let's show our entry's file type!
    ///                 println!("{}: {:?}", entry.path(), file_type);
    ///             } else {
    ///                 println!("Couldn't get file type for {}", entry.path());
    ///             }
    ///         }
    ///     }
    /// }
    /// ```
    #[inline]
    pub fn file_type(&self) -> io::Result<fs::FileType> {
        self.inner.file_type()
    }
    /// Returns the bare file name of this directory entry without any other
    /// leading path component.
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8Path;
    ///
    /// if let Ok(entries) = Utf8Path::new(".").read_dir_utf8() {
    ///     for entry in entries {
    ///         if let Ok(entry) = entry {
    ///             // Here, `entry` is a `DirEntry`.
    ///             println!("{}", entry.file_name());
    ///         }
    ///     }
    /// }
    /// ```
    pub fn file_name(&self) -> &str {
        self.path
            .file_name()
            .expect("path created through DirEntry must have a filename")
    }
    /// Returns the original [`fs::DirEntry`] within this [`Utf8DirEntry`].
    #[inline]
    pub fn into_inner(self) -> fs::DirEntry {
        self.inner
    }
    /// Returns the full path to the file that this entry represents.
    ///
    /// This is analogous to [`path`], but moves ownership of the path.
    ///
    /// [`path`]: struct.Utf8DirEntry.html#method.path
    #[inline]
    #[must_use = "`self` will be dropped if the result is not used"]
    pub fn into_path(self) -> Utf8PathBuf {
        self.path
    }
}
