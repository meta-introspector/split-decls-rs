use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Handle {
    /// Construct a handle from a path.
    ///
    /// Note that the underlying [`File`] is opened in read-only mode on all
    /// platforms.
    ///
    /// [`File`]: https://doc.rust-lang.org/std/fs/struct.File.html
    ///
    /// # Errors
    /// This method will return an [`io::Error`] if the path cannot
    /// be opened, or the file's metadata cannot be obtained.
    /// The most common reasons for this are: the path does not
    /// exist, or there were not enough permissions.
    ///
    /// [`io::Error`]: https://doc.rust-lang.org/std/io/struct.Error.html
    ///
    /// # Examples
    /// Check that two paths are not the same file:
    ///
    /// ```rust,no_run
    /// # use std::error::Error;
    /// use same_file::Handle;
    ///
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// let source = Handle::from_path("./source")?;
    /// let target = Handle::from_path("./target")?;
    /// assert_ne!(source, target, "The files are the same.");
    /// # Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     try_main().unwrap();
    /// # }
    /// ```
    pub fn from_path<P: AsRef<Path>>(p: P) -> io::Result<Handle> {
        imp::Handle::from_path(p).map(Handle)
    }
    /// Construct a handle from a file.
    ///
    /// # Errors
    /// This method will return an [`io::Error`] if the metadata for
    /// the given [`File`] cannot be obtained.
    ///
    /// [`io::Error`]: https://doc.rust-lang.org/std/io/struct.Error.html
    /// [`File`]: https://doc.rust-lang.org/std/fs/struct.File.html
    ///
    /// # Examples
    /// Check that two files are not in fact the same file:
    ///
    /// ```rust,no_run
    /// # use std::error::Error;
    /// # use std::fs::File;
    /// use same_file::Handle;
    ///
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// let source = File::open("./source")?;
    /// let target = File::open("./target")?;
    ///
    /// assert_ne!(
    ///     Handle::from_file(source)?,
    ///     Handle::from_file(target)?,
    ///     "The files are the same."
    /// );
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     try_main().unwrap();
    /// # }
    /// ```
    pub fn from_file(file: File) -> io::Result<Handle> {
        imp::Handle::from_file(file).map(Handle)
    }
    /// Construct a handle from stdin.
    ///
    /// # Errors
    /// This method will return an [`io::Error`] if stdin cannot
    /// be opened due to any I/O-related reason.
    ///
    /// [`io::Error`]: https://doc.rust-lang.org/std/io/struct.Error.html
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use std::error::Error;
    /// use same_file::Handle;
    ///
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// let stdin = Handle::stdin()?;
    /// let stdout = Handle::stdout()?;
    /// let stderr = Handle::stderr()?;
    ///
    /// if stdin == stdout {
    ///     println!("stdin == stdout");
    /// }
    /// if stdin == stderr {
    ///     println!("stdin == stderr");
    /// }
    /// if stdout == stderr {
    ///     println!("stdout == stderr");
    /// }
    /// #
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     try_main().unwrap();
    /// # }
    /// ```
    ///
    /// The output differs depending on the platform.
    ///
    /// On Linux:
    ///
    /// ```text
    /// $ ./example
    /// stdin == stdout
    /// stdin == stderr
    /// stdout == stderr
    /// $ ./example > result
    /// $ cat result
    /// stdin == stderr
    /// $ ./example > result 2>&1
    /// $ cat result
    /// stdout == stderr
    /// ```
    ///
    /// Windows:
    ///
    /// ```text
    /// > example
    /// > example > result 2>&1
    /// > type result
    /// stdout == stderr
    /// ```
    pub fn stdin() -> io::Result<Handle> {
        imp::Handle::stdin().map(Handle)
    }
    /// Construct a handle from stdout.
    ///
    /// # Errors
    /// This method will return an [`io::Error`] if stdout cannot
    /// be opened due to any I/O-related reason.
    ///
    /// [`io::Error`]: https://doc.rust-lang.org/std/io/struct.Error.html
    ///
    /// # Examples
    /// See the example for [`stdin()`].
    ///
    /// [`stdin()`]: #method.stdin
    pub fn stdout() -> io::Result<Handle> {
        imp::Handle::stdout().map(Handle)
    }
    /// Construct a handle from stderr.
    ///
    /// # Errors
    /// This method will return an [`io::Error`] if stderr cannot
    /// be opened due to any I/O-related reason.
    ///
    /// [`io::Error`]: https://doc.rust-lang.org/std/io/struct.Error.html
    ///
    /// # Examples
    /// See the example for [`stdin()`].
    ///
    /// [`stdin()`]: #method.stdin
    pub fn stderr() -> io::Result<Handle> {
        imp::Handle::stderr().map(Handle)
    }
    /// Return a reference to the underlying file.
    ///
    /// # Examples
    /// Ensure that the target file is not the same as the source one,
    /// and copy the data to it:
    ///
    /// ```rust,no_run
    /// # use std::error::Error;
    /// use std::io::prelude::*;
    /// use std::io::Write;
    /// use std::fs::File;
    /// use same_file::Handle;
    ///
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// let source = File::open("source")?;
    /// let target = File::create("target")?;
    ///
    /// let source_handle = Handle::from_file(source)?;
    /// let mut target_handle = Handle::from_file(target)?;
    /// assert_ne!(source_handle, target_handle, "The files are the same.");
    ///
    /// let mut source = source_handle.as_file();
    /// let target = target_handle.as_file_mut();
    ///
    /// let mut buffer = Vec::new();
    /// // data copy is simplified for the purposes of the example
    /// source.read_to_end(&mut buffer)?;
    /// target.write_all(&buffer)?;
    /// #
    /// #    Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #    try_main().unwrap();
    /// # }
    /// ```
    pub fn as_file(&self) -> &File {
        self.0.as_file()
    }
    /// Return a mutable reference to the underlying file.
    ///
    /// # Examples
    /// See the example for [`as_file()`].
    ///
    /// [`as_file()`]: #method.as_file
    pub fn as_file_mut(&mut self) -> &mut File {
        self.0.as_file_mut()
    }
    /// Return the underlying device number of this handle.
    ///
    /// Note that this only works on unix platforms.
    #[cfg(unix)]
    pub fn dev(&self) -> u64 {
        self.0.dev()
    }
    /// Return the underlying inode number of this handle.
    ///
    /// Note that this only works on unix platforms.
    #[cfg(unix)]
    pub fn ino(&self) -> u64 {
        self.0.ino()
    }
}
