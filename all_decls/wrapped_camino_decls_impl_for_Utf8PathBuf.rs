use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Utf8PathBuf {
    /// Allocates an empty [`Utf8PathBuf`].
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8PathBuf;
    ///
    /// let path = Utf8PathBuf::new();
    /// ```
    #[must_use]
    pub fn new() -> Utf8PathBuf {
        Utf8PathBuf(PathBuf::new())
    }
    /// Creates a new [`Utf8PathBuf`] from a [`PathBuf`] containing valid UTF-8 characters.
    ///
    /// Errors with the original [`PathBuf`] if it is not valid UTF-8.
    ///
    /// For a version that returns a type that implements [`std::error::Error`],
    /// see [`TryFrom<&PathBuf>`][tryfrom].
    ///
    /// [tryfrom]: #impl-TryFrom<PathBuf>-for-Utf8PathBuf
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8PathBuf;
    /// use std::ffi::OsStr;
    /// # #[cfg(unix)]
    /// use std::os::unix::ffi::OsStrExt;
    /// use std::path::PathBuf;
    ///
    /// let unicode_path = PathBuf::from("/valid/unicode");
    /// Utf8PathBuf::from_path_buf(unicode_path).expect("valid Unicode path succeeded");
    ///
    /// // Paths on Unix can be non-UTF-8.
    /// # #[cfg(unix)]
    /// let non_unicode_str = OsStr::from_bytes(b"\xFF\xFF\xFF");
    /// # #[cfg(unix)]
    /// let non_unicode_path = PathBuf::from(non_unicode_str);
    /// # #[cfg(unix)]
    /// Utf8PathBuf::from_path_buf(non_unicode_path).expect_err("non-Unicode path failed");
    /// ```
    pub fn from_path_buf(path: PathBuf) -> Result<Utf8PathBuf, PathBuf> {
        match path.into_os_string().into_string() {
            Ok(string) => Ok(Utf8PathBuf::from(string)),
            Err(os_string) => Err(PathBuf::from(os_string)),
        }
    }
    /// Creates a new [`Utf8PathBuf`] from an [`OsString`] containing valid UTF-8 characters.
    ///
    /// Errors with the original [`OsString`] if it is not valid UTF-8.
    ///
    /// For a version that returns a type that implements [`std::error::Error`], use the
    /// [`TryFrom<OsString>`] impl.
    ///
    /// [`TryFrom<OsString>`]: #impl-TryFrom<OsString>-for-Utf8PathBuf
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(osstring_from_str)] {
    /// use camino::Utf8PathBuf;
    /// use std::ffi::OsStr;
    /// use std::ffi::OsString;
    /// use std::convert::TryFrom;
    /// use std::str::FromStr;
    /// # #[cfg(unix)]
    /// use std::os::unix::ffi::OsStrExt;
    ///
    /// let unicode_string = OsString::from_str("/valid/unicode").unwrap();
    /// Utf8PathBuf::from_os_string(unicode_string).expect("valid Unicode path succeeded");
    ///
    /// // Paths on Unix can be non-UTF-8.
    /// # #[cfg(unix)]
    /// let non_unicode_string = OsStr::from_bytes(b"\xFF\xFF\xFF").into();
    /// # #[cfg(unix)]
    /// Utf8PathBuf::from_os_string(non_unicode_string).expect_err("non-Unicode path failed");
    /// # }
    /// ```
    pub fn from_os_string(os_string: OsString) -> Result<Utf8PathBuf, OsString> {
        match os_string.into_string() {
            Ok(string) => Ok(Utf8PathBuf::from(string)),
            Err(os_string) => Err(os_string),
        }
    }
    /// Converts a [`Utf8PathBuf`] to a [`PathBuf`].
    ///
    /// This is equivalent to the [`From<Utf8PathBuf> for PathBuf`][from] implementation,
    /// but may aid in type inference.
    ///
    /// [from]: #impl-From<Utf8PathBuf>-for-PathBuf
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8PathBuf;
    /// use std::path::PathBuf;
    ///
    /// let utf8_path_buf = Utf8PathBuf::from("foo.txt");
    /// let std_path_buf = utf8_path_buf.into_std_path_buf();
    /// assert_eq!(std_path_buf.to_str(), Some("foo.txt"));
    ///
    /// // Convert back to a Utf8PathBuf.
    /// let new_utf8_path_buf = Utf8PathBuf::from_path_buf(std_path_buf).unwrap();
    /// assert_eq!(new_utf8_path_buf, "foo.txt");
    /// ```
    #[must_use = "`self` will be dropped if the result is not used"]
    pub fn into_std_path_buf(self) -> PathBuf {
        self.into()
    }
    /// Creates a new [`Utf8PathBuf`] with a given capacity used to create the internal [`PathBuf`].
    /// See [`with_capacity`] defined on [`PathBuf`].
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8PathBuf;
    ///
    /// let mut path = Utf8PathBuf::with_capacity(10);
    /// let capacity = path.capacity();
    ///
    /// // This push is done without reallocating
    /// path.push(r"C:\");
    ///
    /// assert_eq!(capacity, path.capacity());
    /// ```
    ///
    /// [`with_capacity`]: PathBuf::with_capacity
    #[allow(clippy::incompatible_msrv)]
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Utf8PathBuf {
        Utf8PathBuf(PathBuf::with_capacity(capacity))
    }
    /// Coerces to a [`Utf8Path`] slice.
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::{Utf8Path, Utf8PathBuf};
    ///
    /// let p = Utf8PathBuf::from("/test");
    /// assert_eq!(Utf8Path::new("/test"), p.as_path());
    /// ```
    #[must_use]
    pub fn as_path(&self) -> &Utf8Path {
        unsafe { Utf8Path::assume_utf8(&self.0) }
    }
    /// Consumes and leaks the [`Utf8PathBuf`], returning a mutable reference to the contents,
    /// `&'a mut Utf8Path`.
    ///
    /// The caller has free choice over the returned lifetime, including 'static.
    /// Indeed, this function is ideally used for data that lives for the remainder of
    /// the program’s life, as dropping the returned reference will cause a memory leak.
    ///
    /// It does not reallocate or shrink the [`Utf8PathBuf`], so the leaked allocation may include
    /// unused capacity that is not part of the returned slice. If you want to discard excess
    /// capacity, call [`into_boxed_path`], and then [`Box::leak`] instead.
    /// However, keep in mind that trimming the capacity may result in a reallocation and copy.
    ///
    /// *Requires Rust 1.89 or newer.*
    ///
    /// [`into_boxed_path`]: Self::into_boxed_path
    #[cfg(os_string_pathbuf_leak)]
    #[allow(clippy::incompatible_msrv)]
    #[inline]
    pub fn leak<'a>(self) -> &'a mut Utf8Path {
        unsafe { Utf8Path::assume_utf8_mut(self.0.leak()) }
    }
    /// Extends `self` with `path`.
    ///
    /// If `path` is absolute, it replaces the current path.
    ///
    /// On Windows:
    ///
    /// * if `path` has a root but no prefix (e.g., `\windows`), it
    ///   replaces everything except for the prefix (if any) of `self`.
    /// * if `path` has a prefix but no root, it replaces `self`.
    ///
    /// # Examples
    ///
    /// Pushing a relative path extends the existing path:
    ///
    /// ```
    /// use camino::Utf8PathBuf;
    ///
    /// let mut path = Utf8PathBuf::from("/tmp");
    /// path.push("file.bk");
    /// assert_eq!(path, Utf8PathBuf::from("/tmp/file.bk"));
    /// ```
    ///
    /// Pushing an absolute path replaces the existing path:
    ///
    /// ```
    /// use camino::Utf8PathBuf;
    ///
    /// let mut path = Utf8PathBuf::from("/tmp");
    /// path.push("/etc");
    /// assert_eq!(path, Utf8PathBuf::from("/etc"));
    /// ```
    pub fn push(&mut self, path: impl AsRef<Utf8Path>) {
        self.0.push(&path.as_ref().0)
    }
    /// Truncates `self` to [`self.parent`].
    ///
    /// Returns `false` and does nothing if [`self.parent`] is [`None`].
    /// Otherwise, returns `true`.
    ///
    /// [`self.parent`]: Utf8Path::parent
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::{Utf8Path, Utf8PathBuf};
    ///
    /// let mut p = Utf8PathBuf::from("/spirited/away.rs");
    ///
    /// p.pop();
    /// assert_eq!(Utf8Path::new("/spirited"), p);
    /// p.pop();
    /// assert_eq!(Utf8Path::new("/"), p);
    /// ```
    pub fn pop(&mut self) -> bool {
        self.0.pop()
    }
    /// Updates [`self.file_name`] to `file_name`.
    ///
    /// If [`self.file_name`] was [`None`], this is equivalent to pushing
    /// `file_name`.
    ///
    /// Otherwise it is equivalent to calling [`pop`] and then pushing
    /// `file_name`. The new path will be a sibling of the original path.
    /// (That is, it will have the same parent.)
    ///
    /// [`self.file_name`]: Utf8Path::file_name
    /// [`pop`]: Utf8PathBuf::pop
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8PathBuf;
    ///
    /// let mut buf = Utf8PathBuf::from("/");
    /// assert_eq!(buf.file_name(), None);
    /// buf.set_file_name("bar");
    /// assert_eq!(buf, Utf8PathBuf::from("/bar"));
    /// assert!(buf.file_name().is_some());
    /// buf.set_file_name("baz.txt");
    /// assert_eq!(buf, Utf8PathBuf::from("/baz.txt"));
    /// ```
    pub fn set_file_name(&mut self, file_name: impl AsRef<str>) {
        self.0.set_file_name(file_name.as_ref())
    }
    /// Updates [`self.extension`] to `extension`.
    ///
    /// Returns `false` and does nothing if [`self.file_name`] is [`None`],
    /// returns `true` and updates the extension otherwise.
    ///
    /// If [`self.extension`] is [`None`], the extension is added; otherwise
    /// it is replaced.
    ///
    /// [`self.file_name`]: Utf8Path::file_name
    /// [`self.extension`]: Utf8Path::extension
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::{Utf8Path, Utf8PathBuf};
    ///
    /// let mut p = Utf8PathBuf::from("/feel/the");
    ///
    /// p.set_extension("force");
    /// assert_eq!(Utf8Path::new("/feel/the.force"), p.as_path());
    ///
    /// p.set_extension("dark_side");
    /// assert_eq!(Utf8Path::new("/feel/the.dark_side"), p.as_path());
    /// ```
    pub fn set_extension(&mut self, extension: impl AsRef<str>) -> bool {
        self.0.set_extension(extension.as_ref())
    }
    /// Consumes the [`Utf8PathBuf`], yielding its internal [`String`] storage.
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8PathBuf;
    ///
    /// let p = Utf8PathBuf::from("/the/head");
    /// let s = p.into_string();
    /// assert_eq!(s, "/the/head");
    /// ```
    #[must_use = "`self` will be dropped if the result is not used"]
    pub fn into_string(self) -> String {
        self.into_os_string().into_string().unwrap()
    }
    /// Consumes the [`Utf8PathBuf`], yielding its internal [`OsString`] storage.
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8PathBuf;
    /// use std::ffi::OsStr;
    ///
    /// let p = Utf8PathBuf::from("/the/head");
    /// let s = p.into_os_string();
    /// assert_eq!(s, OsStr::new("/the/head"));
    /// ```
    #[must_use = "`self` will be dropped if the result is not used"]
    pub fn into_os_string(self) -> OsString {
        self.0.into_os_string()
    }
    /// Converts this [`Utf8PathBuf`] into a [boxed](Box) [`Utf8Path`].
    #[must_use = "`self` will be dropped if the result is not used"]
    pub fn into_boxed_path(self) -> Box<Utf8Path> {
        let ptr = Box::into_raw(self.0.into_boxed_path()) as *mut Utf8Path;
        unsafe { Box::from_raw(ptr) }
    }
    /// Invokes [`capacity`] on the underlying instance of [`PathBuf`].
    ///
    /// [`capacity`]: PathBuf::capacity
    #[allow(clippy::incompatible_msrv)]
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }
    /// Invokes [`clear`] on the underlying instance of [`PathBuf`].
    ///
    /// [`clear`]: PathBuf::clear
    #[allow(clippy::incompatible_msrv)]
    pub fn clear(&mut self) {
        self.0.clear()
    }
    /// Invokes [`reserve`] on the underlying instance of [`PathBuf`].
    ///
    /// [`reserve`]: PathBuf::reserve
    #[allow(clippy::incompatible_msrv)]
    pub fn reserve(&mut self, additional: usize) {
        self.0.reserve(additional)
    }
    /// Invokes [`try_reserve`] on the underlying instance of [`PathBuf`].
    ///
    /// *Requires Rust 1.63 or newer.*
    ///
    /// [`try_reserve`]: PathBuf::try_reserve
    #[cfg(try_reserve_2)]
    #[allow(clippy::incompatible_msrv)]
    #[inline]
    pub fn try_reserve(
        &mut self,
        additional: usize,
    ) -> Result<(), std::collections::TryReserveError> {
        self.0.try_reserve(additional)
    }
    /// Invokes [`reserve_exact`] on the underlying instance of [`PathBuf`].
    ///
    /// [`reserve_exact`]: PathBuf::reserve_exact
    #[allow(clippy::incompatible_msrv)]
    pub fn reserve_exact(&mut self, additional: usize) {
        self.0.reserve_exact(additional)
    }
    /// Invokes [`try_reserve_exact`] on the underlying instance of [`PathBuf`].
    ///
    /// *Requires Rust 1.63 or newer.*
    ///
    /// [`try_reserve_exact`]: PathBuf::try_reserve_exact
    #[cfg(try_reserve_2)]
    #[allow(clippy::incompatible_msrv)]
    #[inline]
    pub fn try_reserve_exact(
        &mut self,
        additional: usize,
    ) -> Result<(), std::collections::TryReserveError> {
        self.0.try_reserve_exact(additional)
    }
    /// Invokes [`shrink_to_fit`] on the underlying instance of [`PathBuf`].
    ///
    /// [`shrink_to_fit`]: PathBuf::shrink_to_fit
    #[allow(clippy::incompatible_msrv)]
    pub fn shrink_to_fit(&mut self) {
        self.0.shrink_to_fit()
    }
    /// Invokes [`shrink_to`] on the underlying instance of [`PathBuf`].
    ///
    /// [`shrink_to`]: PathBuf::shrink_to
    #[allow(clippy::incompatible_msrv)]
    #[inline]
    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.0.shrink_to(min_capacity)
    }
}
