use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl MmapOptions {
    /// Creates a new set of options for configuring and creating a memory map.
    ///
    /// # Example
    ///
    /// ```
    /// use memmap2::{MmapMut, MmapOptions};
    /// # use std::io::Result;
    ///
    /// # fn main() -> Result<()> {
    /// // Create a new memory map builder.
    /// let mut mmap_options = MmapOptions::new();
    ///
    /// // Configure the memory map builder using option setters, then create
    /// // a memory map using one of `mmap_options.map_anon`, `mmap_options.map`,
    /// // `mmap_options.map_mut`, `mmap_options.map_exec`, or `mmap_options.map_copy`:
    /// let mut mmap: MmapMut = mmap_options.len(36).map_anon()?;
    ///
    /// // Use the memory map:
    /// mmap.copy_from_slice(b"...data to copy to the memory map...");
    /// # Ok(())
    /// # }
    /// ```
    pub fn new() -> MmapOptions {
        MmapOptions::default()
    }
    /// Configures the memory map to start at byte `offset` from the beginning of the file.
    ///
    /// This option has no effect on anonymous memory maps.
    ///
    /// By default, the offset is 0.
    ///
    /// # Example
    ///
    /// ```
    /// use memmap2::MmapOptions;
    /// use std::fs::File;
    ///
    /// # fn main() -> std::io::Result<()> {
    /// let mmap = unsafe {
    ///     MmapOptions::new()
    ///                 .offset(30)
    ///                 .map(&File::open("LICENSE-APACHE")?)?
    /// };
    /// assert_eq!(&b"Apache License"[..],
    ///            &mmap[..14]);
    /// # Ok(())
    /// # }
    /// ```
    pub fn offset(&mut self, offset: u64) -> &mut Self {
        self.offset = offset;
        self
    }
    /// Configures the created memory mapped buffer to be `len` bytes long.
    ///
    /// This option is mandatory for anonymous memory maps.
    ///
    /// For file-backed memory maps, the length will default to the file length.
    ///
    /// # Example
    ///
    /// ```
    /// use memmap2::MmapOptions;
    /// use std::fs::File;
    ///
    /// # fn main() -> std::io::Result<()> {
    /// let mmap = unsafe {
    ///     MmapOptions::new()
    ///                 .len(9)
    ///                 .map(&File::open("README.md")?)?
    /// };
    /// assert_eq!(&b"# memmap2"[..], &mmap[..]);
    /// # Ok(())
    /// # }
    /// ```
    pub fn len(&mut self, len: usize) -> &mut Self {
        self.len = Some(len);
        self
    }
    fn validate_len(len: u64) -> Result<usize> {
        if isize::try_from(len).is_err() {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "memory map length overflows isize",
            ));
        }
        Ok(len as usize)
    }
    /// Returns the configured length, or the length of the provided file.
    fn get_len<T: MmapAsRawDesc>(&self, file: &T) -> Result<usize> {
        let len = if let Some(len) = self.len {
            len as u64
        } else {
            let desc = file.as_raw_desc();
            let file_len = file_len(desc.0)?;
            if file_len < self.offset {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    "memory map offset is larger than length",
                ));
            }
            file_len - self.offset
        };
        Self::validate_len(len)
    }
    /// Configures the anonymous memory map to be suitable for a process or thread stack.
    ///
    /// This option corresponds to the `MAP_STACK` flag on Linux. It has no effect on Windows.
    ///
    /// This option has no effect on file-backed memory maps.
    ///
    /// # Example
    ///
    /// ```
    /// use memmap2::MmapOptions;
    ///
    /// # fn main() -> std::io::Result<()> {
    /// let stack = MmapOptions::new().stack().len(4096).map_anon();
    /// # Ok(())
    /// # }
    /// ```
    pub fn stack(&mut self) -> &mut Self {
        self.stack = true;
        self
    }
    /// Configures the anonymous memory map to be allocated using huge pages.
    ///
    /// This option corresponds to the `MAP_HUGETLB` flag on Linux. It has no effect on Windows.
    ///
    /// The size of the requested page can be specified in page bits. If not provided, the system
    /// default is requested. The requested length should be a multiple of this, or the mapping
    /// will fail.
    ///
    /// This option has no effect on file-backed memory maps.
    ///
    /// # Example
    ///
    /// ```
    /// use memmap2::MmapOptions;
    ///
    /// # fn main() -> std::io::Result<()> {
    /// let stack = MmapOptions::new().huge(Some(21)).len(2*1024*1024).map_anon();
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// The number 21 corresponds to `MAP_HUGE_2MB`. See mmap(2) for more details.
    pub fn huge(&mut self, page_bits: Option<u8>) -> &mut Self {
        self.huge = Some(page_bits.unwrap_or(0));
        self
    }
    /// Populate (prefault) page tables for a mapping.
    ///
    /// For a file mapping, this causes read-ahead on the file. This will help to reduce blocking on page faults later.
    ///
    /// This option corresponds to the `MAP_POPULATE` flag on Linux. It has no effect on Windows.
    ///
    /// # Example
    ///
    /// ```
    /// use memmap2::MmapOptions;
    /// use std::fs::File;
    ///
    /// # fn main() -> std::io::Result<()> {
    /// let file = File::open("LICENSE-MIT")?;
    ///
    /// let mmap = unsafe {
    ///     MmapOptions::new().populate().map(&file)?
    /// };
    ///
    /// assert_eq!(&b"Copyright"[..], &mmap[..9]);
    /// # Ok(())
    /// # }
    /// ```
    pub fn populate(&mut self) -> &mut Self {
        self.populate = true;
        self
    }
    /// Do not reserve swap space for the memory map.
    ///
    /// By default, platforms may reserve swap space for memory maps.
    /// This guarantees that a write to the mapped memory will succeed, even if physical memory is exhausted.
    /// Otherwise, the write to memory could fail (on Linux with a segfault).
    ///
    /// This option requests that no swap space will be allocated for the memory map,
    /// which can be useful for extremely large maps that are only written to sparsely.
    ///
    /// This option is currently supported on Linux, Android, Apple platforms (macOS, iOS, visionOS, etc.), NetBSD, Solaris and Illumos.
    /// On those platforms, this option corresponds to the `MAP_NORESERVE` flag.
    /// On Linux, this option is ignored if [`vm.overcommit_memory`](https://www.kernel.org/doc/Documentation/vm/overcommit-accounting) is set to 2.
    ///
    /// # Example
    ///
    /// ```
    /// use memmap2::MmapOptions;
    /// use std::fs::File;
    ///
    /// # fn main() -> std::io::Result<()> {
    /// let file = File::open("LICENSE-MIT")?;
    ///
    /// let mmap = unsafe {
    ///     MmapOptions::new().no_reserve_swap().map_copy(&file)?
    /// };
    ///
    /// assert_eq!(&b"Copyright"[..], &mmap[..9]);
    /// # Ok(())
    /// # }
    /// ```
    pub fn no_reserve_swap(&mut self) -> &mut Self {
        self.no_reserve_swap = true;
        self
    }
    /// Creates a read-only memory map backed by a file.
    ///
    /// # Safety
    ///
    /// See the [type-level][MmapOptions] docs for why this function is unsafe.
    ///
    /// # Errors
    ///
    /// This method returns an error when the underlying system call fails, which can happen for a
    /// variety of reasons, such as when the file is not open with read permissions.
    ///
    /// # Example
    ///
    /// ```
    /// use memmap2::MmapOptions;
    /// use std::fs::File;
    /// use std::io::Read;
    ///
    /// # fn main() -> std::io::Result<()> {
    /// let mut file = File::open("LICENSE-APACHE")?;
    ///
    /// let mut contents = Vec::new();
    /// file.read_to_end(&mut contents)?;
    ///
    /// let mmap = unsafe {
    ///     MmapOptions::new().map(&file)?
    /// };
    ///
    /// assert_eq!(&contents[..], &mmap[..]);
    /// # Ok(())
    /// # }
    /// ```
    pub unsafe fn map<T: MmapAsRawDesc>(&self, file: T) -> Result<Mmap> {
        let desc = file.as_raw_desc();
        MmapInner::map(
            self.get_len(&file)?,
            desc.0,
            self.offset,
            self.populate,
            self.no_reserve_swap,
        )
        .map(|inner| Mmap { inner })
    }
    /// Creates a readable and executable memory map backed by a file.
    ///
    /// # Safety
    ///
    /// See the [type-level][MmapOptions] docs for why this function is unsafe.
    ///
    /// # Errors
    ///
    /// This method returns an error when the underlying system call fails, which can happen for a
    /// variety of reasons, such as when the file is not open with read permissions.
    pub unsafe fn map_exec<T: MmapAsRawDesc>(&self, file: T) -> Result<Mmap> {
        let desc = file.as_raw_desc();
        MmapInner::map_exec(
            self.get_len(&file)?,
            desc.0,
            self.offset,
            self.populate,
            self.no_reserve_swap,
        )
        .map(|inner| Mmap { inner })
    }
    /// Creates a writeable memory map backed by a file.
    ///
    /// # Safety
    ///
    /// See the [type-level][MmapOptions] docs for why this function is unsafe.
    ///
    /// # Errors
    ///
    /// This method returns an error when the underlying system call fails, which can happen for a
    /// variety of reasons, such as when the file is not open with read and write permissions.
    ///
    /// # Example
    ///
    /// ```
    /// use std::fs::OpenOptions;
    /// use std::path::PathBuf;
    ///
    /// use memmap2::MmapOptions;
    /// #
    /// # fn main() -> std::io::Result<()> {
    /// # let tempdir = tempfile::tempdir()?;
    /// let path: PathBuf = /* path to file */
    /// #   tempdir.path().join("map_mut");
    /// let file = OpenOptions::new().read(true).write(true).create(true).truncate(true).open(&path)?;
    /// file.set_len(13)?;
    ///
    /// let mut mmap = unsafe {
    ///     MmapOptions::new().map_mut(&file)?
    /// };
    ///
    /// mmap.copy_from_slice(b"Hello, world!");
    /// # Ok(())
    /// # }
    /// ```
    pub unsafe fn map_mut<T: MmapAsRawDesc>(&self, file: T) -> Result<MmapMut> {
        let desc = file.as_raw_desc();
        MmapInner::map_mut(
            self.get_len(&file)?,
            desc.0,
            self.offset,
            self.populate,
            self.no_reserve_swap,
        )
        .map(|inner| MmapMut { inner })
    }
    /// Creates a copy-on-write memory map backed by a file.
    ///
    /// Data written to the memory map will not be visible by other processes,
    /// and will not be carried through to the underlying file.
    ///
    /// # Safety
    ///
    /// See the [type-level][MmapOptions] docs for why this function is unsafe.
    ///
    /// # Errors
    ///
    /// This method returns an error when the underlying system call fails, which can happen for a
    /// variety of reasons, such as when the file is not open with writable permissions.
    ///
    /// # Example
    ///
    /// ```
    /// use memmap2::MmapOptions;
    /// use std::fs::File;
    /// use std::io::Write;
    ///
    /// # fn main() -> std::io::Result<()> {
    /// let file = File::open("LICENSE-APACHE")?;
    /// let mut mmap = unsafe { MmapOptions::new().map_copy(&file)? };
    /// (&mut mmap[..]).write_all(b"Hello, world!")?;
    /// # Ok(())
    /// # }
    /// ```
    pub unsafe fn map_copy<T: MmapAsRawDesc>(&self, file: T) -> Result<MmapMut> {
        let desc = file.as_raw_desc();
        MmapInner::map_copy(
            self.get_len(&file)?,
            desc.0,
            self.offset,
            self.populate,
            self.no_reserve_swap,
        )
        .map(|inner| MmapMut { inner })
    }
    /// Creates a copy-on-write read-only memory map backed by a file.
    ///
    /// # Safety
    ///
    /// See the [type-level][MmapOptions] docs for why this function is unsafe.
    ///
    /// # Errors
    ///
    /// This method returns an error when the underlying system call fails, which can happen for a
    /// variety of reasons, such as when the file is not open with read permissions.
    ///
    /// # Example
    ///
    /// ```
    /// use memmap2::MmapOptions;
    /// use std::fs::File;
    /// use std::io::Read;
    ///
    /// # fn main() -> std::io::Result<()> {
    /// let mut file = File::open("README.md")?;
    ///
    /// let mut contents = Vec::new();
    /// file.read_to_end(&mut contents)?;
    ///
    /// let mmap = unsafe {
    ///     MmapOptions::new().map_copy_read_only(&file)?
    /// };
    ///
    /// assert_eq!(&contents[..], &mmap[..]);
    /// # Ok(())
    /// # }
    /// ```
    pub unsafe fn map_copy_read_only<T: MmapAsRawDesc>(&self, file: T) -> Result<Mmap> {
        let desc = file.as_raw_desc();
        MmapInner::map_copy_read_only(
            self.get_len(&file)?,
            desc.0,
            self.offset,
            self.populate,
            self.no_reserve_swap,
        )
        .map(|inner| Mmap { inner })
    }
    /// Creates an anonymous memory map.
    ///
    /// The memory map length should be configured using [`MmapOptions::len()`]
    /// before creating an anonymous memory map, otherwise a zero-length mapping
    /// will be crated.
    ///
    /// # Errors
    ///
    /// This method returns an error when the underlying system call fails or
    /// when `len > isize::MAX`.
    pub fn map_anon(&self) -> Result<MmapMut> {
        let len = self.len.unwrap_or(0);
        let len = Self::validate_len(len as u64)?;
        MmapInner::map_anon(
            len,
            self.stack,
            self.populate,
            self.huge,
            self.no_reserve_swap,
        )
        .map(|inner| MmapMut { inner })
    }
    /// Creates a raw memory map.
    ///
    /// # Errors
    ///
    /// This method returns an error when the underlying system call fails, which can happen for a
    /// variety of reasons, such as when the file is not open with read and write permissions.
    pub fn map_raw<T: MmapAsRawDesc>(&self, file: T) -> Result<MmapRaw> {
        let desc = file.as_raw_desc();
        MmapInner::map_mut(
            self.get_len(&file)?,
            desc.0,
            self.offset,
            self.populate,
            self.no_reserve_swap,
        )
        .map(|inner| MmapRaw { inner })
    }
    /// Creates a read-only raw memory map
    ///
    /// This is primarily useful to avoid intermediate `Mmap` instances when
    /// read-only access to files modified elsewhere are required.
    ///
    /// # Errors
    ///
    /// This method returns an error when the underlying system call fails
    pub fn map_raw_read_only<T: MmapAsRawDesc>(&self, file: T) -> Result<MmapRaw> {
        let desc = file.as_raw_desc();
        MmapInner::map(
            self.get_len(&file)?,
            desc.0,
            self.offset,
            self.populate,
            self.no_reserve_swap,
        )
        .map(|inner| MmapRaw { inner })
    }
}
