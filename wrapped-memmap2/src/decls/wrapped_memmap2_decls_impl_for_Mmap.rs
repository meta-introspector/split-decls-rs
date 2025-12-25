use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Mmap {
    /// Creates a read-only memory map backed by a file.
    ///
    /// This is equivalent to calling `MmapOptions::new().map(file)`.
    ///
    /// # Safety
    ///
    /// See the [type-level][Mmap] docs for why this function is unsafe.
    ///
    /// # Errors
    ///
    /// This method returns an error when the underlying system call fails, which can happen for a
    /// variety of reasons, such as when the file is not open with read permissions.
    ///
    /// # Example
    ///
    /// ```
    /// use std::fs::File;
    /// use std::io::Read;
    ///
    /// use memmap2::Mmap;
    ///
    /// # fn main() -> std::io::Result<()> {
    /// let mut file = File::open("LICENSE-APACHE")?;
    ///
    /// let mut contents = Vec::new();
    /// file.read_to_end(&mut contents)?;
    ///
    /// let mmap = unsafe { Mmap::map(&file)?  };
    ///
    /// assert_eq!(&contents[..], &mmap[..]);
    /// # Ok(())
    /// # }
    /// ```
    pub unsafe fn map<T: MmapAsRawDesc>(file: T) -> Result<Mmap> {
        MmapOptions::new().map(file)
    }
    /// Transition the memory map to be writable.
    ///
    /// If the memory map is file-backed, the file must have been opened with write permissions.
    ///
    /// # Errors
    ///
    /// This method returns an error when the underlying system call fails, which can happen for a
    /// variety of reasons, such as when the file is not open with writable permissions.
    ///
    /// # Example
    ///
    /// ```
    /// use memmap2::Mmap;
    /// use std::ops::DerefMut;
    /// use std::io::Write;
    /// # use std::fs::OpenOptions;
    ///
    /// # fn main() -> std::io::Result<()> {
    /// # let tempdir = tempfile::tempdir()?;
    /// let file = /* file opened with write permissions */
    /// #          OpenOptions::new()
    /// #                      .read(true)
    /// #                      .write(true)
    /// #                      .create(true)
    /// #                      .truncate(true)
    /// #                      .open(tempdir.path()
    /// #                      .join("make_mut"))?;
    /// # file.set_len(128)?;
    /// let mmap = unsafe { Mmap::map(&file)? };
    /// // ... use the read-only memory map ...
    /// let mut mut_mmap = mmap.make_mut()?;
    /// mut_mmap.deref_mut().write_all(b"hello, world!")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn make_mut(mut self) -> Result<MmapMut> {
        self.inner.make_mut()?;
        Ok(MmapMut { inner: self.inner })
    }
    /// Advise OS how this memory map will be accessed.
    ///
    /// Only supported on Unix.
    ///
    /// See [madvise()](https://man7.org/linux/man-pages/man2/madvise.2.html) map page.
    #[cfg(unix)]
    pub fn advise(&self, advice: Advice) -> Result<()> {
        self.inner.advise(advice as libc::c_int, 0, self.inner.len())
    }
    /// Advise OS how this memory map will be accessed.
    ///
    /// Used with the [unchecked flags][UncheckedAdvice]. Only supported on Unix.
    ///
    /// See [madvise()](https://man7.org/linux/man-pages/man2/madvise.2.html) map page.
    #[cfg(unix)]
    pub unsafe fn unchecked_advise(&self, advice: UncheckedAdvice) -> Result<()> {
        self.inner.advise(advice as libc::c_int, 0, self.inner.len())
    }
    /// Advise OS how this range of memory map will be accessed.
    ///
    /// Only supported on Unix.
    ///
    /// The offset and length must be in the bounds of the memory map.
    ///
    /// See [madvise()](https://man7.org/linux/man-pages/man2/madvise.2.html) map page.
    #[cfg(unix)]
    pub fn advise_range(&self, advice: Advice, offset: usize, len: usize) -> Result<()> {
        self.inner.advise(advice as libc::c_int, offset, len)
    }
    /// Advise OS how this range of memory map will be accessed.
    ///
    /// Used with the [unchecked flags][UncheckedAdvice]. Only supported on Unix.
    ///
    /// The offset and length must be in the bounds of the memory map.
    ///
    /// See [madvise()](https://man7.org/linux/man-pages/man2/madvise.2.html) map page.
    #[cfg(unix)]
    pub unsafe fn unchecked_advise_range(
        &self,
        advice: UncheckedAdvice,
        offset: usize,
        len: usize,
    ) -> Result<()> {
        self.inner.advise(advice as libc::c_int, offset, len)
    }
    /// Lock the whole memory map into RAM. Only supported on Unix.
    ///
    /// See [mlock()](https://man7.org/linux/man-pages/man2/mlock.2.html) map page.
    #[cfg(unix)]
    pub fn lock(&self) -> Result<()> {
        self.inner.lock()
    }
    /// Unlock the whole memory map. Only supported on Unix.
    ///
    /// See [munlock()](https://man7.org/linux/man-pages/man2/munlock.2.html) map page.
    #[cfg(unix)]
    pub fn unlock(&self) -> Result<()> {
        self.inner.unlock()
    }
    /// Adjust the size of the memory mapping.
    ///
    /// This will try to resize the memory mapping in place. If
    /// [`RemapOptions::may_move`] is specified it will move the mapping if it
    /// could not resize in place, otherwise it will error.
    ///
    /// Only supported on Linux.
    ///
    /// See the [`mremap(2)`] man page.
    ///
    /// # Safety
    ///
    /// Resizing the memory mapping beyond the end of the mapped file will
    /// result in UB should you happen to access memory beyond the end of the
    /// file.
    ///
    /// [`mremap(2)`]: https://man7.org/linux/man-pages/man2/mremap.2.html
    #[cfg(target_os = "linux")]
    pub unsafe fn remap(&mut self, new_len: usize, options: RemapOptions) -> Result<()> {
        self.inner.remap(new_len, options)
    }
}
