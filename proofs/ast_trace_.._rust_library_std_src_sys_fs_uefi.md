# AST Trace: ../rust/library/std/src/sys/fs/uefi.rs

Generated 31 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
use r_efi::protocols::file;

use crate::ffi::OsString;
use crate::fmt;
use crate::fs::TryLockError;
use crate::hash::Hash;
use crate::io::{self, BorrowedCursor, IoSlice, IoSliceMut, SeekFrom};
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
use crate::path::{Path, PathBuf};
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=STRUCT | NAME=File(!); | COMPLEXITY=2 | LINES=13

```rust
use crate::sys::time::SystemTime;
use crate::sys::unsupported;

#[expect(dead_code)]
const FILE_PERMISSIONS_MASK: u64 = r_efi::protocols::file::READ_ONLY;

pub struct File(!);

#[derive(Clone)]
pub struct FileAttr {
    attr: u64,
    size: u64,
}
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=STRUCT | NAME=ReadDir(!); | COMPLEXITY=2 | LINES=12

```rust
pub struct ReadDir(!);

pub struct DirEntry(!);

#[derive(Clone, Debug)]
pub struct OpenOptions {
    mode: u64,
    append: bool,
    truncate: bool,
    create_new: bool,
}
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=STRUCT | NAME=FileTimes | COMPLEXITY=2 | LINES=3

```rust
#[derive(Copy, Clone, Debug, Default)]
pub struct FileTimes {}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=FUNCTION | NAME=FilePermissions(bool); | COMPLEXITY=13 | LINES=37

```rust
#[derive(Clone, PartialEq, Eq, Debug)]
// Bool indicates if file is readonly
pub struct FilePermissions(bool);

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
// Bool indicates if directory
pub struct FileType(bool);

#[derive(Debug)]
pub struct DirBuilder;

impl FileAttr {
    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn perm(&self) -> FilePermissions {
        FilePermissions::from_attr(self.attr)
    }

    pub fn file_type(&self) -> FileType {
        FileType::from_attr(self.attr)
    }

    pub fn modified(&self) -> io::Result<SystemTime> {
        unsupported()
    }

    pub fn accessed(&self) -> io::Result<SystemTime> {
        unsupported()
    }

    pub fn created(&self) -> io::Result<SystemTime> {
        unsupported()
    }
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=FUNCTION | NAME=readonly | COMPLEXITY=10 | LINES=19

```rust
impl FilePermissions {
    pub fn readonly(&self) -> bool {
        self.0
    }

    pub fn set_readonly(&mut self, readonly: bool) {
        self.0 = readonly
    }

    const fn from_attr(attr: u64) -> Self {
        Self(attr & r_efi::protocols::file::READ_ONLY != 0)
    }

    #[expect(dead_code)]
    const fn to_attr(&self) -> u64 {
        if self.0 { r_efi::protocols::file::READ_ONLY } else { 0 }
    }
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=FUNCTION | NAME=set_accessed | COMPLEXITY=4 | LINES=5

```rust
impl FileTimes {
    pub fn set_accessed(&mut self, _t: SystemTime) {}
    pub fn set_modified(&mut self, _t: SystemTime) {}
}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=FUNCTION | NAME=is_dir | COMPLEXITY=6 | LINES=19

```rust
impl FileType {
    pub fn is_dir(&self) -> bool {
        self.0
    }

    pub fn is_file(&self) -> bool {
        !self.is_dir()
    }

    // Symlinks are not supported in UEFI
    pub fn is_symlink(&self) -> bool {
        false
    }

    const fn from_attr(attr: u64) -> Self {
        Self(attr & r_efi::protocols::file::DIRECTORY != 0)
    }
}
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=5 | LINES=6

```rust
impl fmt::Debug for ReadDir {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0
    }
}
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=FUNCTION | NAME=next | COMPLEXITY=5 | LINES=8

```rust
impl Iterator for ReadDir {
    type Item = io::Result<DirEntry>;

    fn next(&mut self) -> Option<io::Result<DirEntry>> {
        self.0
    }
}
```

## Block 12
**Metadata**: AST_ID=12 | TYPE=FUNCTION | NAME=path | COMPLEXITY=6 | LINES=18

```rust
impl DirEntry {
    pub fn path(&self) -> PathBuf {
        self.0
    }

    pub fn file_name(&self) -> OsString {
        self.0
    }

    pub fn metadata(&self) -> io::Result<FileAttr> {
        self.0
    }

    pub fn file_type(&self) -> io::Result<FileType> {
        self.0
    }
}
```

## Block 13
**Metadata**: AST_ID=13 | TYPE=FUNCTION | NAME=new | COMPLEXITY=29 | LINES=56

```rust
impl OpenOptions {
    pub fn new() -> OpenOptions {
        OpenOptions { mode: 0, append: false, create_new: false, truncate: false }
    }

    pub fn read(&mut self, read: bool) {
        if read {
            self.mode |= file::MODE_READ;
        } else {
            self.mode &= !file::MODE_READ;
        }
    }

    pub fn write(&mut self, write: bool) {
        if write {
            // Valid Combinations: Read, Read/Write, Read/Write/Create
            self.read(true);
            self.mode |= file::MODE_WRITE;
        } else {
            self.mode &= !file::MODE_WRITE;
        }
    }

    pub fn append(&mut self, append: bool) {
        // Docs state that `.write(true).append(true)` has the same effect as `.append(true)`
        if append {
            self.write(true);
        }
        self.append = append;
    }

    pub fn truncate(&mut self, truncate: bool) {
        self.truncate = truncate;
    }

    pub fn create(&mut self, create: bool) {
        if create {
            self.mode |= file::MODE_CREATE;
        } else {
            self.mode &= !file::MODE_CREATE;
        }
    }

    pub fn create_new(&mut self, create_new: bool) {
        self.create_new = create_new;
    }

    #[expect(dead_code)]
    const fn is_mode_valid(&self) -> bool {
        // Valid Combinations: Read, Read/Write, Read/Write/Create
        self.mode == file::MODE_READ
            || self.mode == (file::MODE_READ | file::MODE_WRITE)
            || self.mode == (file::MODE_READ | file::MODE_WRITE | file::MODE_CREATE)
    }
}
```

## Block 14
**Metadata**: AST_ID=14 | TYPE=FUNCTION | NAME=open | COMPLEXITY=29 | LINES=98

```rust
impl File {
    pub fn open(_path: &Path, _opts: &OpenOptions) -> io::Result<File> {
        unsupported()
    }

    pub fn file_attr(&self) -> io::Result<FileAttr> {
        self.0
    }

    pub fn fsync(&self) -> io::Result<()> {
        self.0
    }

    pub fn datasync(&self) -> io::Result<()> {
        self.0
    }

    pub fn lock(&self) -> io::Result<()> {
        self.0
    }

    pub fn lock_shared(&self) -> io::Result<()> {
        self.0
    }

    pub fn try_lock(&self) -> Result<(), TryLockError> {
        self.0
    }

    pub fn try_lock_shared(&self) -> Result<(), TryLockError> {
        self.0
    }

    pub fn unlock(&self) -> io::Result<()> {
        self.0
    }

    pub fn truncate(&self, _size: u64) -> io::Result<()> {
        self.0
    }

    pub fn read(&self, _buf: &mut [u8]) -> io::Result<usize> {
        self.0
    }

    pub fn read_vectored(&self, _bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
        self.0
    }

    pub fn is_read_vectored(&self) -> bool {
        self.0
    }

    pub fn read_buf(&self, _cursor: BorrowedCursor<'_>) -> io::Result<()> {
        self.0
    }

    pub fn write(&self, _buf: &[u8]) -> io::Result<usize> {
        self.0
    }

    pub fn write_vectored(&self, _bufs: &[IoSlice<'_>]) -> io::Result<usize> {
        self.0
    }

    pub fn is_write_vectored(&self) -> bool {
        self.0
    }

    pub fn flush(&self) -> io::Result<()> {
        self.0
    }

    pub fn seek(&self, _pos: SeekFrom) -> io::Result<u64> {
        self.0
    }

    pub fn size(&self) -> Option<io::Result<u64>> {
        self.0
    }

    pub fn tell(&self) -> io::Result<u64> {
        self.0
    }

    pub fn duplicate(&self) -> io::Result<File> {
        self.0
    }

    pub fn set_permissions(&self, _perm: FilePermissions) -> io::Result<()> {
        self.0
    }

    pub fn set_times(&self, _times: FileTimes) -> io::Result<()> {
        self.0
    }
}
```

## Block 15
**Metadata**: AST_ID=15 | TYPE=FUNCTION | NAME=new | COMPLEXITY=4 | LINES=10

```rust
impl DirBuilder {
    pub fn new() -> DirBuilder {
        DirBuilder
    }

    pub fn mkdir(&self, p: &Path) -> io::Result<()> {
        uefi_fs::mkdir(p)
    }
}
```

## Block 16
**Metadata**: AST_ID=16 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=5 | LINES=6

```rust
impl fmt::Debug for File {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0
    }
}
```

## Block 17
**Metadata**: AST_ID=17 | TYPE=FUNCTION | NAME=readdir | COMPLEXITY=2 | LINES=4

```rust
pub fn readdir(_p: &Path) -> io::Result<ReadDir> {
    unsupported()
}
```

## Block 18
**Metadata**: AST_ID=18 | TYPE=FUNCTION | NAME=unlink | COMPLEXITY=2 | LINES=4

```rust
pub fn unlink(_p: &Path) -> io::Result<()> {
    unsupported()
}
```

## Block 19
**Metadata**: AST_ID=19 | TYPE=FUNCTION | NAME=rename | COMPLEXITY=2 | LINES=4

```rust
pub fn rename(_old: &Path, _new: &Path) -> io::Result<()> {
    unsupported()
}
```

## Block 20
**Metadata**: AST_ID=20 | TYPE=FUNCTION | NAME=set_perm | COMPLEXITY=2 | LINES=4

```rust
pub fn set_perm(_p: &Path, _perm: FilePermissions) -> io::Result<()> {
    unsupported()
}
```

## Block 21
**Metadata**: AST_ID=21 | TYPE=FUNCTION | NAME=rmdir | COMPLEXITY=2 | LINES=4

```rust
pub fn rmdir(_p: &Path) -> io::Result<()> {
    unsupported()
}
```

## Block 22
**Metadata**: AST_ID=22 | TYPE=FUNCTION | NAME=remove_dir_all | COMPLEXITY=2 | LINES=4

```rust
pub fn remove_dir_all(_path: &Path) -> io::Result<()> {
    unsupported()
}
```

## Block 23
**Metadata**: AST_ID=23 | TYPE=FUNCTION | NAME=exists | COMPLEXITY=8 | LINES=9

```rust
pub fn exists(path: &Path) -> io::Result<bool> {
    let f = uefi_fs::File::from_path(path, r_efi::protocols::file::MODE_READ, 0);
    match f {
        Ok(_) => Ok(true),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(false),
        Err(e) => Err(e),
    }
}
```

## Block 24
**Metadata**: AST_ID=24 | TYPE=FUNCTION | NAME=readlink | COMPLEXITY=2 | LINES=4

```rust
pub fn readlink(_p: &Path) -> io::Result<PathBuf> {
    unsupported()
}
```

## Block 25
**Metadata**: AST_ID=25 | TYPE=FUNCTION | NAME=symlink | COMPLEXITY=2 | LINES=4

```rust
pub fn symlink(_original: &Path, _link: &Path) -> io::Result<()> {
    unsupported()
}
```

## Block 26
**Metadata**: AST_ID=26 | TYPE=FUNCTION | NAME=link | COMPLEXITY=2 | LINES=4

```rust
pub fn link(_src: &Path, _dst: &Path) -> io::Result<()> {
    unsupported()
}
```

## Block 27
**Metadata**: AST_ID=27 | TYPE=FUNCTION | NAME=stat | COMPLEXITY=2 | LINES=4

```rust
pub fn stat(_p: &Path) -> io::Result<FileAttr> {
    unsupported()
}
```

## Block 28
**Metadata**: AST_ID=28 | TYPE=FUNCTION | NAME=lstat | COMPLEXITY=2 | LINES=4

```rust
pub fn lstat(p: &Path) -> io::Result<FileAttr> {
    stat(p)
}
```

## Block 29
**Metadata**: AST_ID=29 | TYPE=FUNCTION | NAME=canonicalize | COMPLEXITY=2 | LINES=4

```rust
pub fn canonicalize(p: &Path) -> io::Result<PathBuf> {
    crate::path::absolute(p)
}
```

## Block 30
**Metadata**: AST_ID=30 | TYPE=FUNCTION | NAME=copy | COMPLEXITY=2 | LINES=4

```rust
pub fn copy(_from: &Path, _to: &Path) -> io::Result<u64> {
    unsupported()
}
```

## Block 31
**Metadata**: AST_ID=31 | TYPE=FUNCTION | NAME=open_volume_from_device_path | COMPLEXITY=85 | LINES=157

```rust
mod uefi_fs {
    use r_efi::protocols::{device_path, file, simple_file_system};

    use crate::boxed::Box;
    use crate::io;
    use crate::path::Path;
    use crate::ptr::NonNull;
    use crate::sys::helpers;

    pub(crate) struct File(NonNull<file::Protocol>);

    impl File {
        pub(crate) fn from_path(path: &Path, open_mode: u64, attr: u64) -> io::Result<Self> {
            let absolute = crate::path::absolute(path)?;

            let p = helpers::OwnedDevicePath::from_text(absolute.as_os_str())?;
            let (vol, mut path_remaining) = Self::open_volume_from_device_path(p.borrow())?;

            vol.open(&mut path_remaining, open_mode, attr)
        }

        /// Open Filesystem volume given a devicepath to the volume, or a file/directory in the
        /// volume. The path provided should be absolute UEFI device path, without any UEFI shell
        /// mappings.
        ///
        /// Returns
        /// 1. The volume as a UEFI File
        /// 2. Path relative to the volume.
        ///
        /// For example, given "PciRoot(0x0)/Pci(0x1,0x1)/Ata(Secondary,Slave,0x0)/\abc\run.efi",
        /// this will open the volume "PciRoot(0x0)/Pci(0x1,0x1)/Ata(Secondary,Slave,0x0)"
        /// and return the remaining file path "\abc\run.efi".
        fn open_volume_from_device_path(
            path: helpers::BorrowedDevicePath<'_>,
        ) -> io::Result<(Self, Box<[u16]>)> {
            let handles = match helpers::locate_handles(simple_file_system::PROTOCOL_GUID) {
                Ok(x) => x,
                Err(e) => return Err(e),
            };
            for handle in handles {
                let volume_device_path: NonNull<device_path::Protocol> =
                    match helpers::open_protocol(handle, device_path::PROTOCOL_GUID) {
                        Ok(x) => x,
                        Err(_) => continue,
                    };
                let volume_device_path = helpers::BorrowedDevicePath::new(volume_device_path);

                if let Some(left_path) = path_best_match(&volume_device_path, &path) {
                    return Ok((Self::open_volume(handle)?, left_path));
                }
            }

            Err(io::const_error!(io::ErrorKind::NotFound, "Volume Not Found"))
        }

        // Open volume on device_handle using SIMPLE_FILE_SYSTEM_PROTOCOL
        fn open_volume(device_handle: NonNull<crate::ffi::c_void>) -> io::Result<Self> {
            let simple_file_system_protocol = helpers::open_protocol::<simple_file_system::Protocol>(
                device_handle,
                simple_file_system::PROTOCOL_GUID,
            )?;

            let mut file_protocol = crate::ptr::null_mut();
            let r = unsafe {
                ((*simple_file_system_protocol.as_ptr()).open_volume)(
                    simple_file_system_protocol.as_ptr(),
                    &mut file_protocol,
                )
            };
            if r.is_error() {
                return Err(io::Error::from_raw_os_error(r.as_usize()));
            }

            // Since no error was returned, file protocol should be non-NULL.
            let p = NonNull::new(file_protocol).unwrap();
            Ok(Self(p))
        }

        fn open(&self, path: &mut [u16], open_mode: u64, attr: u64) -> io::Result<Self> {
            let file_ptr = self.0.as_ptr();
            let mut file_opened = crate::ptr::null_mut();

            let r = unsafe {
                ((*file_ptr).open)(file_ptr, &mut file_opened, path.as_mut_ptr(), open_mode, attr)
            };

            if r.is_error() {
                return Err(io::Error::from_raw_os_error(r.as_usize()));
            }

            // Since no error was returned, file protocol should be non-NULL.
            let p = NonNull::new(file_opened).unwrap();
            Ok(File(p))
        }
    }

    impl Drop for File {
        fn drop(&mut self) {
            let file_ptr = self.0.as_ptr();
            let _ = unsafe { ((*self.0.as_ptr()).close)(file_ptr) };
        }
    }

    /// A helper to check that target path is a descendent of source. It is expected to be used with
    /// absolute UEFI device paths without any UEFI shell mappings.
    ///
    /// Returns the path relative to source
    ///
    /// For example, given "PciRoot(0x0)/Pci(0x1,0x1)/Ata(Secondary,Slave,0x0)/" and
    /// "PciRoot(0x0)/Pci(0x1,0x1)/Ata(Secondary,Slave,0x0)/\abc\run.efi", this will return
    /// "\abc\run.efi"
    fn path_best_match(
        source: &helpers::BorrowedDevicePath<'_>,
        target: &helpers::BorrowedDevicePath<'_>,
    ) -> Option<Box<[u16]>> {
        let mut source_iter = source.iter().take_while(|x| !x.is_end_instance());
        let mut target_iter = target.iter().take_while(|x| !x.is_end_instance());

        loop {
            match (source_iter.next(), target_iter.next()) {
                (Some(x), Some(y)) if x == y => continue,
                (None, Some(y)) => {
                    let p = y.to_path().to_text().ok()?;
                    return helpers::os_string_to_raw(&p);
                }
                _ => return None,
            }
        }
    }

    /// An implementation of mkdir to allow creating new directory without having to open the
    /// volume twice (once for checking and once for creating)
    pub(crate) fn mkdir(path: &Path) -> io::Result<()> {
        let absolute = crate::path::absolute(path)?;

        let p = helpers::OwnedDevicePath::from_text(absolute.as_os_str())?;
        let (vol, mut path_remaining) = File::open_volume_from_device_path(p.borrow())?;

        // Check if file exists
        match vol.open(&mut path_remaining, file::MODE_READ, 0) {
            Ok(_) => {
                return Err(io::Error::new(io::ErrorKind::AlreadyExists, "Path already exists"));
            }
            Err(e) if e.kind() == io::ErrorKind::NotFound => {}
            Err(e) => return Err(e),
        }

        let _ = vol.open(
            &mut path_remaining,
            file::MODE_READ | file::MODE_WRITE | file::MODE_CREATE,
            file::DIRECTORY,
        )?;

        Ok(())
    }
}
```

---
*Generated by AST tracing system*
