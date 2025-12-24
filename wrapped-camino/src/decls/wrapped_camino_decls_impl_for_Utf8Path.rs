use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Utf8Path {
    /// Directly wraps a string slice as a [`Utf8Path`] slice.
    ///
    /// This is a cost-free conversion.
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8Path;
    ///
    /// Utf8Path::new("foo.txt");
    /// ```
    ///
    /// You can create [`Utf8Path`]s from [`String`]s, or even other [`Utf8Path`]s:
    ///
    /// ```
    /// use camino::Utf8Path;
    ///
    /// let string = String::from("foo.txt");
    /// let from_string = Utf8Path::new(&string);
    /// let from_path = Utf8Path::new(&from_string);
    /// assert_eq!(from_string, from_path);
    /// ```
    pub fn new(s: &(impl AsRef<str> + ?Sized)) -> &Utf8Path {
        let path = Path::new(s.as_ref());
        unsafe { Utf8Path::assume_utf8(path) }
    }
    /// Converts a [`Path`] to a [`Utf8Path`].
    ///
    /// Returns [`None`] if the path is not valid UTF-8.
    ///
    /// For a version that returns a type that implements [`std::error::Error`],
    /// see [`TryFrom<&Path>`][tryfrom].
    ///
    /// [tryfrom]: #impl-TryFrom<%26Path>-for-%26Utf8Path
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8Path;
    /// use std::ffi::OsStr;
    /// # #[cfg(unix)]
    /// use std::os::unix::ffi::OsStrExt;
    /// use std::path::Path;
    ///
    /// let unicode_path = Path::new("/valid/unicode");
    /// Utf8Path::from_path(unicode_path).expect("valid Unicode path succeeded");
    ///
    /// // Paths on Unix can be non-UTF-8.
    /// # #[cfg(unix)]
    /// let non_unicode_str = OsStr::from_bytes(b"\xFF\xFF\xFF");
    /// # #[cfg(unix)]
    /// let non_unicode_path = Path::new(non_unicode_str);
    /// # #[cfg(unix)]
    /// assert!(Utf8Path::from_path(non_unicode_path).is_none(), "non-Unicode path failed");
    /// ```
    pub fn from_path(path: &Path) -> Option<&Utf8Path> {
        path.as_os_str().to_str().map(Utf8Path::new)
    }
    /// Converts an [`OsStr`] to a [`Utf8Path`].
    ///
    /// Returns [`None`] if the path is not valid UTF-8.
    ///
    /// For a version that returns a type that implements [`std::error::Error`], use the
    /// [`TryFrom<&OsStr>`][tryfrom] impl.
    ///
    /// [tryfrom]: #impl-TryFrom<%26OsStr>-for-%26Utf8Path
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8Path;
    /// use std::ffi::OsStr;
    /// # #[cfg(unix)]
    /// use std::os::unix::ffi::OsStrExt;
    /// use std::path::Path;
    ///
    /// let unicode_string = OsStr::new("/valid/unicode");
    /// Utf8Path::from_os_str(unicode_string).expect("valid Unicode string succeeded");
    ///
    /// // Paths on Unix can be non-UTF-8.
    /// # #[cfg(unix)]
    /// let non_unicode_str = OsStr::from_bytes(b"\xFF\xFF\xFF");
    /// # #[cfg(unix)]
    /// assert!(Utf8Path::from_os_str(non_unicode_str).is_none(), "non-Unicode string failed");
    /// ```
    pub fn from_os_str(path: &OsStr) -> Option<&Utf8Path> {
        path.to_str().map(Utf8Path::new)
    }
    /// Converts a [`Utf8Path`] to a [`Path`].
    ///
    /// This is equivalent to the [`AsRef<Path> for Utf8PathBuf`][asref] implementation,
    /// but may aid in type inference.
    ///
    /// [asref]: Utf8PathBuf#impl-AsRef<Path>-for-Utf8PathBuf
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8Path;
    /// use std::path::Path;
    ///
    /// let utf8_path = Utf8Path::new("foo.txt");
    /// let std_path: &Path = utf8_path.as_std_path();
    /// assert_eq!(std_path.to_str(), Some("foo.txt"));
    ///
    /// // Convert back to a Utf8Path.
    /// let new_utf8_path = Utf8Path::from_path(std_path).unwrap();
    /// assert_eq!(new_utf8_path, "foo.txt");
    /// ```
    #[inline]
    pub fn as_std_path(&self) -> &Path {
        self.as_ref()
    }
    /// Yields the underlying [`str`] slice.
    ///
    /// Unlike [`Path::to_str`], this always returns a slice because the contents
    /// of a [`Utf8Path`] are guaranteed to be valid UTF-8.
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8Path;
    ///
    /// let s = Utf8Path::new("foo.txt").as_str();
    /// assert_eq!(s, "foo.txt");
    /// ```
    ///
    /// [`str`]: str
    #[inline]
    #[must_use]
    pub fn as_str(&self) -> &str {
        unsafe { str_assume_utf8(self.as_os_str()) }
    }
    /// Yields the underlying [`OsStr`] slice.
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8Path;
    ///
    /// let os_str = Utf8Path::new("foo.txt").as_os_str();
    /// assert_eq!(os_str, std::ffi::OsStr::new("foo.txt"));
    /// ```
    #[inline]
    #[must_use]
    pub fn as_os_str(&self) -> &OsStr {
        self.0.as_os_str()
    }
    /// Converts a [`Utf8Path`] to an owned [`Utf8PathBuf`].
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::{Utf8Path, Utf8PathBuf};
    ///
    /// let path_buf = Utf8Path::new("foo.txt").to_path_buf();
    /// assert_eq!(path_buf, Utf8PathBuf::from("foo.txt"));
    /// ```
    #[inline]
    #[must_use = "this returns the result of the operation, \
                  without modifying the original"]
    pub fn to_path_buf(&self) -> Utf8PathBuf {
        Utf8PathBuf(self.0.to_path_buf())
    }
    /// Returns `true` if the [`Utf8Path`] is absolute, i.e., if it is independent of
    /// the current directory.
    ///
    /// * On Unix, a path is absolute if it starts with the root, so
    ///   `is_absolute` and [`has_root`] are equivalent.
    ///
    /// * On Windows, a path is absolute if it has a prefix and starts with the
    ///   root: `C:\windows` is absolute, while `C:temp` and `\temp` are not.
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8Path;
    ///
    /// assert!(!Utf8Path::new("foo.txt").is_absolute());
    /// ```
    ///
    /// [`has_root`]: Utf8Path::has_root
    #[inline]
    #[must_use]
    pub fn is_absolute(&self) -> bool {
        self.0.is_absolute()
    }
    /// Returns `true` if the [`Utf8Path`] is relative, i.e., not absolute.
    ///
    /// See [`is_absolute`]'s documentation for more details.
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8Path;
    ///
    /// assert!(Utf8Path::new("foo.txt").is_relative());
    /// ```
    ///
    /// [`is_absolute`]: Utf8Path::is_absolute
    #[inline]
    #[must_use]
    pub fn is_relative(&self) -> bool {
        self.0.is_relative()
    }
    /// Returns `true` if the [`Utf8Path`] has a root.
    ///
    /// * On Unix, a path has a root if it begins with `/`.
    ///
    /// * On Windows, a path has a root if it:
    ///     * has no prefix and begins with a separator, e.g., `\windows`
    ///     * has a prefix followed by a separator, e.g., `C:\windows` but not `C:windows`
    ///     * has any non-disk prefix, e.g., `\\server\share`
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8Path;
    ///
    /// assert!(Utf8Path::new("/etc/passwd").has_root());
    /// ```
    #[inline]
    #[must_use]
    pub fn has_root(&self) -> bool {
        self.0.has_root()
    }
    /// Returns the [`Path`] without its final component, if there is one.
    ///
    /// Returns [`None`] if the path terminates in a root or prefix.
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8Path;
    ///
    /// let path = Utf8Path::new("/foo/bar");
    /// let parent = path.parent().unwrap();
    /// assert_eq!(parent, Utf8Path::new("/foo"));
    ///
    /// let grand_parent = parent.parent().unwrap();
    /// assert_eq!(grand_parent, Utf8Path::new("/"));
    /// assert_eq!(grand_parent.parent(), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn parent(&self) -> Option<&Utf8Path> {
        self.0.parent().map(|path| { unsafe { Utf8Path::assume_utf8(path) } })
    }
    /// Produces an iterator over [`Utf8Path`] and its ancestors.
    ///
    /// The iterator will yield the [`Utf8Path`] that is returned if the [`parent`] method is used zero
    /// or more times. That means, the iterator will yield `&self`, `&self.parent().unwrap()`,
    /// `&self.parent().unwrap().parent().unwrap()` and so on. If the [`parent`] method returns
    /// [`None`], the iterator will do likewise. The iterator will always yield at least one value,
    /// namely `&self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8Path;
    ///
    /// let mut ancestors = Utf8Path::new("/foo/bar").ancestors();
    /// assert_eq!(ancestors.next(), Some(Utf8Path::new("/foo/bar")));
    /// assert_eq!(ancestors.next(), Some(Utf8Path::new("/foo")));
    /// assert_eq!(ancestors.next(), Some(Utf8Path::new("/")));
    /// assert_eq!(ancestors.next(), None);
    ///
    /// let mut ancestors = Utf8Path::new("../foo/bar").ancestors();
    /// assert_eq!(ancestors.next(), Some(Utf8Path::new("../foo/bar")));
    /// assert_eq!(ancestors.next(), Some(Utf8Path::new("../foo")));
    /// assert_eq!(ancestors.next(), Some(Utf8Path::new("..")));
    /// assert_eq!(ancestors.next(), Some(Utf8Path::new("")));
    /// assert_eq!(ancestors.next(), None);
    /// ```
    ///
    /// [`parent`]: Utf8Path::parent
    #[inline]
    pub fn ancestors(&self) -> Utf8Ancestors<'_> {
        Utf8Ancestors(self.0.ancestors())
    }
    /// Returns the final component of the [`Utf8Path`], if there is one.
    ///
    /// If the path is a normal file, this is the file name. If it's the path of a directory, this
    /// is the directory name.
    ///
    /// Returns [`None`] if the path terminates in `..`.
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8Path;
    ///
    /// assert_eq!(Some("bin"), Utf8Path::new("/usr/bin/").file_name());
    /// assert_eq!(Some("foo.txt"), Utf8Path::new("tmp/foo.txt").file_name());
    /// assert_eq!(Some("foo.txt"), Utf8Path::new("foo.txt/.").file_name());
    /// assert_eq!(Some("foo.txt"), Utf8Path::new("foo.txt/.//").file_name());
    /// assert_eq!(None, Utf8Path::new("foo.txt/..").file_name());
    /// assert_eq!(None, Utf8Path::new("/").file_name());
    /// ```
    #[inline]
    #[must_use]
    pub fn file_name(&self) -> Option<&str> {
        self.0.file_name().map(|s| { unsafe { str_assume_utf8(s) } })
    }
    /// Returns a path that, when joined onto `base`, yields `self`.
    ///
    /// # Errors
    ///
    /// If `base` is not a prefix of `self` (i.e., [`starts_with`]
    /// returns `false`), returns [`Err`].
    ///
    /// [`starts_with`]: Utf8Path::starts_with
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::{Utf8Path, Utf8PathBuf};
    ///
    /// let path = Utf8Path::new("/test/haha/foo.txt");
    ///
    /// assert_eq!(path.strip_prefix("/"), Ok(Utf8Path::new("test/haha/foo.txt")));
    /// assert_eq!(path.strip_prefix("/test"), Ok(Utf8Path::new("haha/foo.txt")));
    /// assert_eq!(path.strip_prefix("/test/"), Ok(Utf8Path::new("haha/foo.txt")));
    /// assert_eq!(path.strip_prefix("/test/haha/foo.txt"), Ok(Utf8Path::new("")));
    /// assert_eq!(path.strip_prefix("/test/haha/foo.txt/"), Ok(Utf8Path::new("")));
    ///
    /// assert!(path.strip_prefix("test").is_err());
    /// assert!(path.strip_prefix("/haha").is_err());
    ///
    /// let prefix = Utf8PathBuf::from("/test/");
    /// assert_eq!(path.strip_prefix(prefix), Ok(Utf8Path::new("haha/foo.txt")));
    /// ```
    #[inline]
    pub fn strip_prefix(
        &self,
        base: impl AsRef<Path>,
    ) -> Result<&Utf8Path, StripPrefixError> {
        self.0.strip_prefix(base).map(|path| { unsafe { Utf8Path::assume_utf8(path) } })
    }
    /// Determines whether `base` is a prefix of `self`.
    ///
    /// Only considers whole path components to match.
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8Path;
    ///
    /// let path = Utf8Path::new("/etc/passwd");
    ///
    /// assert!(path.starts_with("/etc"));
    /// assert!(path.starts_with("/etc/"));
    /// assert!(path.starts_with("/etc/passwd"));
    /// assert!(path.starts_with("/etc/passwd/")); // extra slash is okay
    /// assert!(path.starts_with("/etc/passwd///")); // multiple extra slashes are okay
    ///
    /// assert!(!path.starts_with("/e"));
    /// assert!(!path.starts_with("/etc/passwd.txt"));
    ///
    /// assert!(!Utf8Path::new("/etc/foo.rs").starts_with("/etc/foo"));
    /// ```
    #[inline]
    #[must_use]
    pub fn starts_with(&self, base: impl AsRef<Path>) -> bool {
        self.0.starts_with(base)
    }
    /// Determines whether `child` is a suffix of `self`.
    ///
    /// Only considers whole path components to match.
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8Path;
    ///
    /// let path = Utf8Path::new("/etc/resolv.conf");
    ///
    /// assert!(path.ends_with("resolv.conf"));
    /// assert!(path.ends_with("etc/resolv.conf"));
    /// assert!(path.ends_with("/etc/resolv.conf"));
    ///
    /// assert!(!path.ends_with("/resolv.conf"));
    /// assert!(!path.ends_with("conf")); // use .extension() instead
    /// ```
    #[inline]
    #[must_use]
    pub fn ends_with(&self, base: impl AsRef<Path>) -> bool {
        self.0.ends_with(base)
    }
    /// Extracts the stem (non-extension) portion of [`self.file_name`].
    ///
    /// [`self.file_name`]: Utf8Path::file_name
    ///
    /// The stem is:
    ///
    /// * [`None`], if there is no file name;
    /// * The entire file name if there is no embedded `.`;
    /// * The entire file name if the file name begins with `.` and has no other `.`s within;
    /// * Otherwise, the portion of the file name before the final `.`
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8Path;
    ///
    /// assert_eq!("foo", Utf8Path::new("foo.rs").file_stem().unwrap());
    /// assert_eq!("foo.tar", Utf8Path::new("foo.tar.gz").file_stem().unwrap());
    /// ```
    #[inline]
    #[must_use]
    pub fn file_stem(&self) -> Option<&str> {
        self.0.file_stem().map(|s| { unsafe { str_assume_utf8(s) } })
    }
    /// Extracts the extension of [`self.file_name`], if possible.
    ///
    /// The extension is:
    ///
    /// * [`None`], if there is no file name;
    /// * [`None`], if there is no embedded `.`;
    /// * [`None`], if the file name begins with `.` and has no other `.`s within;
    /// * Otherwise, the portion of the file name after the final `.`
    ///
    /// [`self.file_name`]: Utf8Path::file_name
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8Path;
    ///
    /// assert_eq!("rs", Utf8Path::new("foo.rs").extension().unwrap());
    /// assert_eq!("gz", Utf8Path::new("foo.tar.gz").extension().unwrap());
    /// ```
    #[inline]
    #[must_use]
    pub fn extension(&self) -> Option<&str> {
        self.0.extension().map(|s| { unsafe { str_assume_utf8(s) } })
    }
    /// Creates an owned [`Utf8PathBuf`] with `path` adjoined to `self`.
    ///
    /// See [`Utf8PathBuf::push`] for more details on what it means to adjoin a path.
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::{Utf8Path, Utf8PathBuf};
    ///
    /// assert_eq!(Utf8Path::new("/etc").join("passwd"), Utf8PathBuf::from("/etc/passwd"));
    /// ```
    #[inline]
    #[must_use]
    pub fn join(&self, path: impl AsRef<Utf8Path>) -> Utf8PathBuf {
        Utf8PathBuf(self.0.join(&path.as_ref().0))
    }
    /// Creates an owned [`PathBuf`] with `path` adjoined to `self`.
    ///
    /// See [`PathBuf::push`] for more details on what it means to adjoin a path.
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8Path;
    /// use std::path::PathBuf;
    ///
    /// assert_eq!(Utf8Path::new("/etc").join_os("passwd"), PathBuf::from("/etc/passwd"));
    /// ```
    #[inline]
    #[must_use]
    pub fn join_os(&self, path: impl AsRef<Path>) -> PathBuf {
        self.0.join(path)
    }
    /// Creates an owned [`Utf8PathBuf`] like `self` but with the given file name.
    ///
    /// See [`Utf8PathBuf::set_file_name`] for more details.
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::{Utf8Path, Utf8PathBuf};
    ///
    /// let path = Utf8Path::new("/tmp/foo.txt");
    /// assert_eq!(path.with_file_name("bar.txt"), Utf8PathBuf::from("/tmp/bar.txt"));
    ///
    /// let path = Utf8Path::new("/tmp");
    /// assert_eq!(path.with_file_name("var"), Utf8PathBuf::from("/var"));
    /// ```
    #[inline]
    #[must_use]
    pub fn with_file_name(&self, file_name: impl AsRef<str>) -> Utf8PathBuf {
        Utf8PathBuf(self.0.with_file_name(file_name.as_ref()))
    }
    /// Creates an owned [`Utf8PathBuf`] like `self` but with the given extension.
    ///
    /// See [`Utf8PathBuf::set_extension`] for more details.
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::{Utf8Path, Utf8PathBuf};
    ///
    /// let path = Utf8Path::new("foo.rs");
    /// assert_eq!(path.with_extension("txt"), Utf8PathBuf::from("foo.txt"));
    ///
    /// let path = Utf8Path::new("foo.tar.gz");
    /// assert_eq!(path.with_extension(""), Utf8PathBuf::from("foo.tar"));
    /// assert_eq!(path.with_extension("xz"), Utf8PathBuf::from("foo.tar.xz"));
    /// assert_eq!(path.with_extension("").with_extension("txt"), Utf8PathBuf::from("foo.txt"));
    /// ```
    #[inline]
    pub fn with_extension(&self, extension: impl AsRef<str>) -> Utf8PathBuf {
        Utf8PathBuf(self.0.with_extension(extension.as_ref()))
    }
    /// Produces an iterator over the [`Utf8Component`]s of the path.
    ///
    /// When parsing the path, there is a small amount of normalization:
    ///
    /// * Repeated separators are ignored, so `a/b` and `a//b` both have
    ///   `a` and `b` as components.
    ///
    /// * Occurrences of `.` are normalized away, except if they are at the
    ///   beginning of the path. For example, `a/./b`, `a/b/`, `a/b/.` and
    ///   `a/b` all have `a` and `b` as components, but `./a/b` starts with
    ///   an additional [`CurDir`] component.
    ///
    /// * A trailing slash is normalized away, `/a/b` and `/a/b/` are equivalent.
    ///
    /// Note that no other normalization takes place; in particular, `a/c`
    /// and `a/b/../c` are distinct, to account for the possibility that `b`
    /// is a symbolic link (so its parent isn't `a`).
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::{Utf8Component, Utf8Path};
    ///
    /// let mut components = Utf8Path::new("/tmp/foo.txt").components();
    ///
    /// assert_eq!(components.next(), Some(Utf8Component::RootDir));
    /// assert_eq!(components.next(), Some(Utf8Component::Normal("tmp")));
    /// assert_eq!(components.next(), Some(Utf8Component::Normal("foo.txt")));
    /// assert_eq!(components.next(), None)
    /// ```
    ///
    /// [`CurDir`]: Utf8Component::CurDir
    #[inline]
    pub fn components(&self) -> Utf8Components<'_> {
        Utf8Components(self.0.components())
    }
    /// Produces an iterator over the path's components viewed as [`str`]
    /// slices.
    ///
    /// For more information about the particulars of how the path is separated
    /// into components, see [`components`].
    ///
    /// [`components`]: Utf8Path::components
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8Path;
    ///
    /// let mut it = Utf8Path::new("/tmp/foo.txt").iter();
    /// assert_eq!(it.next(), Some(std::path::MAIN_SEPARATOR.to_string().as_str()));
    /// assert_eq!(it.next(), Some("tmp"));
    /// assert_eq!(it.next(), Some("foo.txt"));
    /// assert_eq!(it.next(), None)
    /// ```
    #[inline]
    pub fn iter(&self) -> Iter<'_> {
        Iter { inner: self.components() }
    }
    /// Queries the file system to get information about a file, directory, etc.
    ///
    /// This function will traverse symbolic links to query information about the
    /// destination file.
    ///
    /// This is an alias to [`fs::metadata`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use camino::Utf8Path;
    ///
    /// let path = Utf8Path::new("/Minas/tirith");
    /// let metadata = path.metadata().expect("metadata call failed");
    /// println!("{:?}", metadata.file_type());
    /// ```
    #[inline]
    pub fn metadata(&self) -> io::Result<fs::Metadata> {
        self.0.metadata()
    }
    /// Queries the metadata about a file without following symlinks.
    ///
    /// This is an alias to [`fs::symlink_metadata`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use camino::Utf8Path;
    ///
    /// let path = Utf8Path::new("/Minas/tirith");
    /// let metadata = path.symlink_metadata().expect("symlink_metadata call failed");
    /// println!("{:?}", metadata.file_type());
    /// ```
    #[inline]
    pub fn symlink_metadata(&self) -> io::Result<fs::Metadata> {
        self.0.symlink_metadata()
    }
    /// Returns the canonical, absolute form of the path with all intermediate
    /// components normalized and symbolic links resolved.
    ///
    /// This returns a [`PathBuf`] because even if a symlink is valid Unicode, its target may not
    /// be. For a version that returns a [`Utf8PathBuf`], see
    /// [`canonicalize_utf8`](Self::canonicalize_utf8).
    ///
    /// This is an alias to [`fs::canonicalize`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use camino::Utf8Path;
    /// use std::path::PathBuf;
    ///
    /// let path = Utf8Path::new("/foo/test/../test/bar.rs");
    /// assert_eq!(path.canonicalize().unwrap(), PathBuf::from("/foo/test/bar.rs"));
    /// ```
    #[inline]
    pub fn canonicalize(&self) -> io::Result<PathBuf> {
        self.0.canonicalize()
    }
    /// Returns the canonical, absolute form of the path with all intermediate
    /// components normalized and symbolic links resolved.
    ///
    /// This method attempts to convert the resulting [`PathBuf`] into a [`Utf8PathBuf`]. For a
    /// version that does not attempt to do this conversion, see
    /// [`canonicalize`](Self::canonicalize).
    ///
    /// # Errors
    ///
    /// The I/O operation may return an error: see the [`fs::canonicalize`]
    /// documentation for more.
    ///
    /// If the resulting path is not UTF-8, an [`io::Error`] is returned with the
    /// [`ErrorKind`](io::ErrorKind) set to [`InvalidData`](io::ErrorKind::InvalidData)
    /// and the payload set to a [`FromPathBufError`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use camino::{Utf8Path, Utf8PathBuf};
    ///
    /// let path = Utf8Path::new("/foo/test/../test/bar.rs");
    /// assert_eq!(path.canonicalize_utf8().unwrap(), Utf8PathBuf::from("/foo/test/bar.rs"));
    /// ```
    pub fn canonicalize_utf8(&self) -> io::Result<Utf8PathBuf> {
        self.canonicalize()
            .and_then(|path| path.try_into().map_err(FromPathBufError::into_io_error))
    }
    /// Reads a symbolic link, returning the file that the link points to.
    ///
    /// This returns a [`PathBuf`] because even if a symlink is valid Unicode, its target may not
    /// be. For a version that returns a [`Utf8PathBuf`], see
    /// [`read_link_utf8`](Self::read_link_utf8).
    ///
    /// This is an alias to [`fs::read_link`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use camino::Utf8Path;
    ///
    /// let path = Utf8Path::new("/laputa/sky_castle.rs");
    /// let path_link = path.read_link().expect("read_link call failed");
    /// ```
    #[inline]
    pub fn read_link(&self) -> io::Result<PathBuf> {
        self.0.read_link()
    }
    /// Reads a symbolic link, returning the file that the link points to.
    ///
    /// This method attempts to convert the resulting [`PathBuf`] into a [`Utf8PathBuf`]. For a
    /// version that does not attempt to do this conversion, see [`read_link`](Self::read_link).
    ///
    /// # Errors
    ///
    /// The I/O operation may return an error: see the [`fs::read_link`]
    /// documentation for more.
    ///
    /// If the resulting path is not UTF-8, an [`io::Error`] is returned with the
    /// [`ErrorKind`](io::ErrorKind) set to [`InvalidData`](io::ErrorKind::InvalidData)
    /// and the payload set to a [`FromPathBufError`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use camino::Utf8Path;
    ///
    /// let path = Utf8Path::new("/laputa/sky_castle.rs");
    /// let path_link = path.read_link_utf8().expect("read_link call failed");
    /// ```
    pub fn read_link_utf8(&self) -> io::Result<Utf8PathBuf> {
        self.read_link()
            .and_then(|path| path.try_into().map_err(FromPathBufError::into_io_error))
    }
    /// Returns an iterator over the entries within a directory.
    ///
    /// The iterator will yield instances of [`io::Result`]`<`[`fs::DirEntry`]`>`. New
    /// errors may be encountered after an iterator is initially constructed.
    ///
    /// This is an alias to [`fs::read_dir`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use camino::Utf8Path;
    ///
    /// let path = Utf8Path::new("/laputa");
    /// for entry in path.read_dir().expect("read_dir call failed") {
    ///     if let Ok(entry) = entry {
    ///         println!("{:?}", entry.path());
    ///     }
    /// }
    /// ```
    #[inline]
    pub fn read_dir(&self) -> io::Result<fs::ReadDir> {
        self.0.read_dir()
    }
    /// Returns an iterator over the entries within a directory.
    ///
    /// The iterator will yield instances of [`io::Result`]`<`[`Utf8DirEntry`]`>`. New
    /// errors may be encountered after an iterator is initially constructed.
    ///
    /// # Errors
    ///
    /// The I/O operation may return an error: see the [`fs::read_dir`]
    /// documentation for more.
    ///
    /// If a directory entry is not UTF-8, an [`io::Error`] is returned with the
    /// [`ErrorKind`](io::ErrorKind) set to [`InvalidData`](io::ErrorKind::InvalidData)
    /// and the payload set to a [`FromPathBufError`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use camino::Utf8Path;
    ///
    /// let path = Utf8Path::new("/laputa");
    /// for entry in path.read_dir_utf8().expect("read_dir call failed") {
    ///     if let Ok(entry) = entry {
    ///         println!("{}", entry.path());
    ///     }
    /// }
    /// ```
    #[inline]
    pub fn read_dir_utf8(&self) -> io::Result<ReadDirUtf8> {
        self.0.read_dir().map(|inner| ReadDirUtf8 { inner })
    }
    /// Returns `true` if the path points at an existing entity.
    ///
    /// Warning: this method may be error-prone, consider using [`try_exists()`] instead!
    /// It also has a risk of introducing time-of-check to time-of-use (TOCTOU) bugs.
    ///
    /// This function will traverse symbolic links to query information about the
    /// destination file. In case of broken symbolic links this will return `false`.
    ///
    /// If you cannot access the directory containing the file, e.g., because of a
    /// permission error, this will return `false`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use camino::Utf8Path;
    /// assert!(!Utf8Path::new("does_not_exist.txt").exists());
    /// ```
    ///
    /// # See Also
    ///
    /// This is a convenience function that coerces errors to false. If you want to
    /// check errors, call [`fs::metadata`].
    ///
    /// [`try_exists()`]: Self::try_exists
    #[must_use]
    #[inline]
    pub fn exists(&self) -> bool {
        self.0.exists()
    }
    /// Returns `Ok(true)` if the path points at an existing entity.
    ///
    /// This function will traverse symbolic links to query information about the
    /// destination file. In case of broken symbolic links this will return `Ok(false)`.
    ///
    /// As opposed to the [`exists()`] method, this one doesn't silently ignore errors
    /// unrelated to the path not existing. (E.g. it will return [`Err`] in case of permission
    /// denied on some of the parent directories.)
    ///
    /// Note that while this avoids some pitfalls of the `exists()` method, it still can not
    /// prevent time-of-check to time-of-use (TOCTOU) bugs. You should only use it in scenarios
    /// where those bugs are not an issue.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use camino::Utf8Path;
    /// assert!(
    ///     !Utf8Path::new("does_not_exist.txt")
    ///         .try_exists()
    ///         .expect("Can't check existence of file does_not_exist.txt")
    /// );
    /// assert!(Utf8Path::new("/root/secret_file.txt").try_exists().is_err());
    /// ```
    ///
    /// [`exists()`]: Self::exists
    #[inline]
    pub fn try_exists(&self) -> io::Result<bool> {
        match fs::metadata(self) {
            Ok(_) => Ok(true),
            Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(false),
            Err(error) => Err(error),
        }
    }
    /// Returns `true` if the path exists on disk and is pointing at a regular file.
    ///
    /// This function will traverse symbolic links to query information about the
    /// destination file. In case of broken symbolic links this will return `false`.
    ///
    /// If you cannot access the directory containing the file, e.g., because of a
    /// permission error, this will return `false`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use camino::Utf8Path;
    /// assert_eq!(Utf8Path::new("./is_a_directory/").is_file(), false);
    /// assert_eq!(Utf8Path::new("a_file.txt").is_file(), true);
    /// ```
    ///
    /// # See Also
    ///
    /// This is a convenience function that coerces errors to false. If you want to
    /// check errors, call [`fs::metadata`] and handle its [`Result`]. Then call
    /// [`fs::Metadata::is_file`] if it was [`Ok`].
    ///
    /// When the goal is simply to read from (or write to) the source, the most
    /// reliable way to test the source can be read (or written to) is to open
    /// it. Only using `is_file` can break workflows like `diff <( prog_a )` on
    /// a Unix-like system for example. See [`fs::File::open`] or
    /// [`fs::OpenOptions::open`] for more information.
    #[must_use]
    #[inline]
    pub fn is_file(&self) -> bool {
        self.0.is_file()
    }
    /// Returns `true` if the path exists on disk and is pointing at a directory.
    ///
    /// This function will traverse symbolic links to query information about the
    /// destination file. In case of broken symbolic links this will return `false`.
    ///
    /// If you cannot access the directory containing the file, e.g., because of a
    /// permission error, this will return `false`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use camino::Utf8Path;
    /// assert_eq!(Utf8Path::new("./is_a_directory/").is_dir(), true);
    /// assert_eq!(Utf8Path::new("a_file.txt").is_dir(), false);
    /// ```
    ///
    /// # See Also
    ///
    /// This is a convenience function that coerces errors to false. If you want to
    /// check errors, call [`fs::metadata`] and handle its [`Result`]. Then call
    /// [`fs::Metadata::is_dir`] if it was [`Ok`].
    #[must_use]
    #[inline]
    pub fn is_dir(&self) -> bool {
        self.0.is_dir()
    }
    /// Returns `true` if the path exists on disk and is pointing at a symbolic link.
    ///
    /// This function will not traverse symbolic links.
    /// In case of a broken symbolic link this will also return true.
    ///
    /// If you cannot access the directory containing the file, e.g., because of a
    /// permission error, this will return false.
    ///
    /// # Examples
    ///
    #[cfg_attr(unix, doc = "```no_run")]
    #[cfg_attr(not(unix), doc = "```ignore")]
    /// use camino::Utf8Path;
    /// use std::os::unix::fs::symlink;
    ///
    /// let link_path = Utf8Path::new("link");
    /// symlink("/origin_does_not_exist/", link_path).unwrap();
    /// assert_eq!(link_path.is_symlink(), true);
    /// assert_eq!(link_path.exists(), false);
    /// ```
    ///
    /// # See Also
    ///
    /// This is a convenience function that coerces errors to false. If you want to
    /// check errors, call [`Utf8Path::symlink_metadata`] and handle its [`Result`]. Then call
    /// [`fs::Metadata::is_symlink`] if it was [`Ok`].
    #[must_use]
    pub fn is_symlink(&self) -> bool {
        self.symlink_metadata().map(|m| m.file_type().is_symlink()).unwrap_or(false)
    }
    /// Converts a [`Box<Utf8Path>`] into a [`Utf8PathBuf`] without copying or allocating.
    #[must_use = "`self` will be dropped if the result is not used"]
    #[inline]
    pub fn into_path_buf(self: Box<Utf8Path>) -> Utf8PathBuf {
        let ptr = Box::into_raw(self) as *mut Path;
        let boxed_path = unsafe { Box::from_raw(ptr) };
        Utf8PathBuf(boxed_path.into_path_buf())
    }
    #[inline]
    unsafe fn assume_utf8(path: &Path) -> &Utf8Path {
        &*(path as *const Path as *const Utf8Path)
    }
    #[cfg(path_buf_deref_mut)]
    #[inline]
    unsafe fn assume_utf8_mut(path: &mut Path) -> &mut Utf8Path {
        &mut *(path as *mut Path as *mut Utf8Path)
    }
}
