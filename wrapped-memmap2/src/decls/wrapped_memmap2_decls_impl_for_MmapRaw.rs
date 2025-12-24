use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl MmapRaw {
    /// Creates a writeable memory map backed by a file.
    ///
    /// This is equivalent to calling `MmapOptions::new().map_raw(file)`.
    ///
    /// # Errors
    ///
    /// This method returns an error when the underlying system call fails, which can happen for a
    /// variety of reasons, such as when the file is not open with read and write permissions.
    pub fn map_raw<T: MmapAsRawDesc>(file: T) -> Result<MmapRaw> {
        MmapOptions::new().map_raw(file)
    }
    /// Returns a raw pointer to the memory mapped file.
    ///
    /// Before dereferencing this pointer, you have to make sure that the file has not been
    /// truncated since the memory map was created.
    /// Avoiding this will not introduce memory safety issues in Rust terms,
    /// but will cause SIGBUS (or equivalent) signal.
    #[inline]
    pub fn as_ptr(&self) -> *const u8 {
        self.inner.ptr()
    }
    /// Returns an unsafe mutable pointer to the memory mapped file.
    ///
    /// Before dereferencing this pointer, you have to make sure that the file has not been
    /// truncated since the memory map was created.
    /// Avoiding this will not introduce memory safety issues in Rust terms,
    /// but will cause SIGBUS (or equivalent) signal.
    #[inline]
    pub fn as_mut_ptr(&self) -> *mut u8 {
        self.inner.ptr() as *mut u8
    }
    /// Returns the length in bytes of the memory map.
    ///
    /// Note that truncating the file can cause the length to change (and render this value unusable).
    #[inline]
    pub fn len(&self) -> usize {
        self.inner.len()
    }
    /// Flushes outstanding memory map modifications to disk.
    ///
    /// When this method returns with a non-error result, all outstanding changes to a file-backed
    /// memory map are guaranteed to be durably stored. The file's metadata (including last
    /// modification timestamp) may not be updated.
    ///
    /// # Example
    ///
    /// ```
    /// use std::fs::OpenOptions;
    /// use std::io::Write;
    /// use std::path::PathBuf;
    /// use std::slice;
    ///
    /// use memmap2::MmapRaw;
    ///
    /// # fn main() -> std::io::Result<()> {
    /// let tempdir = tempfile::tempdir()?;
    /// let path: PathBuf = /* path to file */
    /// #   tempdir.path().join("flush");
    /// let file = OpenOptions::new().read(true).write(true).create(true).truncate(true).open(&path)?;
    /// file.set_len(128)?;
    ///
    /// let mut mmap = unsafe { MmapRaw::map_raw(&file)? };
    ///
    /// let mut memory = unsafe { slice::from_raw_parts_mut(mmap.as_mut_ptr(), 128) };
    /// memory.write_all(b"Hello, world!")?;
    /// mmap.flush()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn flush(&self) -> Result<()> {
        let len = self.len();
        self.inner.flush(0, len)
    }
    /// Asynchronously flushes outstanding memory map modifications to disk.
    ///
    /// This method initiates flushing modified pages to durable storage, but it will not wait for
    /// the operation to complete before returning. The file's metadata (including last
    /// modification timestamp) may not be updated.
    pub fn flush_async(&self) -> Result<()> {
        let len = self.len();
        self.inner.flush_async(0, len)
    }
    /// Flushes outstanding memory map modifications in the range to disk.
    ///
    /// The offset and length must be in the bounds of the memory map.
    ///
    /// When this method returns with a non-error result, all outstanding changes to a file-backed
    /// memory in the range are guaranteed to be durable stored. The file's metadata (including
    /// last modification timestamp) may not be updated. It is not guaranteed the only the changes
    /// in the specified range are flushed; other outstanding changes to the memory map may be
    /// flushed as well.
    pub fn flush_range(&self, offset: usize, len: usize) -> Result<()> {
        self.inner.flush(offset, len)
    }
    /// Asynchronously flushes outstanding memory map modifications in the range to disk.
    ///
    /// The offset and length must be in the bounds of the memory map.
    ///
    /// This method initiates flushing modified pages to durable storage, but it will not wait for
    /// the operation to complete before returning. The file's metadata (including last
    /// modification timestamp) may not be updated. It is not guaranteed that the only changes
    /// flushed are those in the specified range; other outstanding changes to the memory map may
    /// be flushed as well.
    pub fn flush_async_range(&self, offset: usize, len: usize) -> Result<()> {
        self.inner.flush_async(offset, len)
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
    /// The offset and length must be in the bounds of the memory map.
    ///
    /// Only supported on Unix.
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
