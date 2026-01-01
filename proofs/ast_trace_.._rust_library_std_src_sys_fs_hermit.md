# AST Trace: ../rust/library/std/src/sys/fs/hermit.rs

Generated 53 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
use crate::ffi::{CStr, OsStr, OsString, c_char};
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
use crate::fs::TryLockError;
use crate::io::{self, BorrowedCursor, Error, ErrorKind, IoSlice, IoSliceMut, SeekFrom};
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=5

```rust
use crate::os::hermit::ffi::OsStringExt;
use crate::os::hermit::hermit_abi::{
    self, DT_DIR, DT_LNK, DT_REG, DT_UNKNOWN, O_APPEND, O_CREAT, O_DIRECTORY, O_EXCL, O_RDONLY,
    O_RDWR, O_TRUNC, O_WRONLY, S_IFDIR, S_IFLNK, S_IFMT, S_IFREG, dirent64, stat as stat_struct,
};
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
use crate::os::hermit::io::{AsFd, AsRawFd, BorrowedFd, FromRawFd, IntoRawFd, RawFd};
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
use crate::path::{Path, PathBuf};
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=4

```rust
use crate::sync::Arc;
use crate::sys::common::small_c_string::run_path_with_cstr;
use crate::sys::fd::FileDesc;
pub use crate::sys::fs::common::{copy, exists};
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
use crate::sys::time::SystemTime;
use crate::sys::{cvt, unsupported, unsupported_err};
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
use crate::sys_common::{AsInner, AsInnerMut, FromInner, IntoInner};
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
use crate::{fmt, mem};
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=STRUCT | NAME=File(FileDesc); | COMPLEXITY=2 | LINES=7

```rust
#[derive(Debug)]
pub struct File(FileDesc);
#[derive(Clone)]
pub struct FileAttr {
    stat_val: stat_struct,
}
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=FUNCTION | NAME=from_stat | COMPLEXITY=4 | LINES=6

```rust
impl FileAttr {
    fn from_stat(stat_val: stat_struct) -> Self {
        Self { stat_val }
    }
}
```

## Block 12
**Metadata**: AST_ID=12 | TYPE=STRUCT | NAME=InnerReadDir | COMPLEXITY=2 | LINES=6

```rust
// all DirEntry's will have a reference to this struct
struct InnerReadDir {
    root: PathBuf,
    dir: Vec<u8>,
}
```

## Block 13
**Metadata**: AST_ID=13 | TYPE=FUNCTION | NAME=new | COMPLEXITY=4 | LINES=6

```rust
impl InnerReadDir {
    pub fn new(root: PathBuf, dir: Vec<u8>) -> Self {
        Self { root, dir }
    }
}
```

## Block 14
**Metadata**: AST_ID=14 | TYPE=STRUCT | NAME=ReadDir | COMPLEXITY=2 | LINES=5

```rust
pub struct ReadDir {
    inner: Arc<InnerReadDir>,
    pos: usize,
}
```

## Block 15
**Metadata**: AST_ID=15 | TYPE=FUNCTION | NAME=new | COMPLEXITY=4 | LINES=6

```rust
impl ReadDir {
    fn new(inner: InnerReadDir) -> Self {
        Self { inner: Arc::new(inner), pos: 0 }
    }
}
```

## Block 16
**Metadata**: AST_ID=16 | TYPE=STRUCT | NAME=DirEntry | COMPLEXITY=2 | LINES=11

```rust
pub struct DirEntry {
    /// path to the entry
    root: PathBuf,
    /// 64-bit inode number
    ino: u64,
    /// File type
    type_: u8,
    /// name of the entry
    name: OsString,
}
```

## Block 17
**Metadata**: AST_ID=17 | TYPE=STRUCT | NAME=OpenOptions | COMPLEXITY=2 | LINES=13

```rust
#[derive(Clone, Debug)]
pub struct OpenOptions {
    // generic
    read: bool,
    write: bool,
    append: bool,
    truncate: bool,
    create: bool,
    create_new: bool,
    // system-specific
    mode: i32,
}
```

## Block 18
**Metadata**: AST_ID=18 | TYPE=STRUCT | NAME=FileTimes | COMPLEXITY=2 | LINES=3

```rust
#[derive(Copy, Clone, Debug, Default)]
pub struct FileTimes {}
```

## Block 19
**Metadata**: AST_ID=19 | TYPE=STRUCT | NAME=FilePermissions | COMPLEXITY=2 | LINES=5

```rust
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FilePermissions {
    mode: u32,
}
```

## Block 20
**Metadata**: AST_ID=20 | TYPE=STRUCT | NAME=FileType | COMPLEXITY=2 | LINES=5

```rust
#[derive(Copy, Clone, Eq, Debug)]
pub struct FileType {
    mode: u8,
}
```

## Block 21
**Metadata**: AST_ID=21 | TYPE=FUNCTION | NAME=eq | COMPLEXITY=5 | LINES=6

```rust
impl PartialEq for FileType {
    fn eq(&self, other: &Self) -> bool {
        self.mode == other.mode
    }
}
```

## Block 22
**Metadata**: AST_ID=22 | TYPE=FUNCTION | NAME=hash | COMPLEXITY=5 | LINES=6

```rust
impl core::hash::Hash for FileType {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.mode.hash(state);
    }
}
```

## Block 23
**Metadata**: AST_ID=23 | TYPE=STRUCT | NAME=DirBuilder | COMPLEXITY=2 | LINES=5

```rust
#[derive(Debug)]
pub struct DirBuilder {
    mode: u32,
}
```

## Block 24
**Metadata**: AST_ID=24 | TYPE=FUNCTION | NAME=modified | COMPLEXITY=15 | LINES=33

```rust
impl FileAttr {
    pub fn modified(&self) -> io::Result<SystemTime> {
        Ok(SystemTime::new(self.stat_val.st_mtim.tv_sec, self.stat_val.st_mtim.tv_nsec))
    }

    pub fn accessed(&self) -> io::Result<SystemTime> {
        Ok(SystemTime::new(self.stat_val.st_atim.tv_sec, self.stat_val.st_atim.tv_nsec))
    }

    pub fn created(&self) -> io::Result<SystemTime> {
        Ok(SystemTime::new(self.stat_val.st_ctim.tv_sec, self.stat_val.st_ctim.tv_nsec))
    }

    pub fn size(&self) -> u64 {
        self.stat_val.st_size as u64
    }

    pub fn perm(&self) -> FilePermissions {
        FilePermissions { mode: self.stat_val.st_mode }
    }

    pub fn file_type(&self) -> FileType {
        let masked_mode = self.stat_val.st_mode & S_IFMT;
        let mode = match masked_mode {
            S_IFDIR => DT_DIR,
            S_IFLNK => DT_LNK,
            S_IFREG => DT_REG,
            _ => DT_UNKNOWN,
        };
        FileType { mode }
    }
}
```

## Block 25
**Metadata**: AST_ID=25 | TYPE=FUNCTION | NAME=readonly | COMPLEXITY=7 | LINES=16

```rust
impl FilePermissions {
    pub fn readonly(&self) -> bool {
        // check if any class (owner, group, others) has write permission
        self.mode & 0o222 == 0
    }

    pub fn set_readonly(&mut self, _readonly: bool) {
        unimplemented!()
    }

    #[allow(dead_code)]
    pub fn mode(&self) -> u32 {
        self.mode as u32
    }
}
```

## Block 26
**Metadata**: AST_ID=26 | TYPE=FUNCTION | NAME=set_accessed | COMPLEXITY=4 | LINES=5

```rust
impl FileTimes {
    pub fn set_accessed(&mut self, _t: SystemTime) {}
    pub fn set_modified(&mut self, _t: SystemTime) {}
}
```

## Block 27
**Metadata**: AST_ID=27 | TYPE=FUNCTION | NAME=is_dir | COMPLEXITY=5 | LINES=12

```rust
impl FileType {
    pub fn is_dir(&self) -> bool {
        self.mode == DT_DIR
    }
    pub fn is_file(&self) -> bool {
        self.mode == DT_REG
    }
    pub fn is_symlink(&self) -> bool {
        self.mode == DT_LNK
    }
}
```

## Block 28
**Metadata**: AST_ID=28 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=5 | LINES=8

```rust
impl fmt::Debug for ReadDir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // This will only be called from std::fs::ReadDir, which will add a "ReadDir()" frame.
        // Thus the result will be e.g. 'ReadDir("/home")'
        fmt::Debug::fmt(&*self.inner.root, f)
    }
}
```

## Block 29
**Metadata**: AST_ID=29 | TYPE=FUNCTION | NAME=next | COMPLEXITY=36 | LINES=43

```rust
impl Iterator for ReadDir {
    type Item = io::Result<DirEntry>;

    fn next(&mut self) -> Option<io::Result<DirEntry>> {
        let mut counter: usize = 0;
        let mut offset: usize = 0;

        // loop over all directory entries and search the entry for the current position
        loop {
            // leave function, if the loop reaches the of the buffer (with all entries)
            if offset >= self.inner.dir.len() {
                return None;
            }

            let dir = unsafe { &*(self.inner.dir.as_ptr().add(offset) as *const dirent64) };

            if counter == self.pos {
                self.pos += 1;

                // After dirent64, the file name is stored. d_reclen represents the length of the dirent64
                // plus the length of the file name. Consequently, file name has a size of d_reclen minus
                // the size of dirent64. The file name is always a C string and terminated by `\0`.
                // Consequently, we are able to ignore the last byte.
                let name_bytes =
                    unsafe { CStr::from_ptr(&dir.d_name as *const _ as *const c_char).to_bytes() };
                let entry = DirEntry {
                    root: self.inner.root.clone(),
                    ino: dir.d_ino,
                    type_: dir.d_type,
                    name: OsString::from_vec(name_bytes.to_vec()),
                };

                return Some(Ok(entry));
            }

            counter += 1;

            // move to the next dirent64, which is directly stored after the previous one
            offset = offset + usize::from(dir.d_reclen);
        }
    }
}
```

## Block 30
**Metadata**: AST_ID=30 | TYPE=FUNCTION | NAME=path | COMPLEXITY=10 | LINES=29

```rust
impl DirEntry {
    pub fn path(&self) -> PathBuf {
        self.root.join(self.file_name_os_str())
    }

    pub fn file_name(&self) -> OsString {
        self.file_name_os_str().to_os_string()
    }

    pub fn metadata(&self) -> io::Result<FileAttr> {
        let mut path = self.path();
        path.set_file_name(self.file_name_os_str());
        lstat(&path)
    }

    pub fn file_type(&self) -> io::Result<FileType> {
        Ok(FileType { mode: self.type_ })
    }

    #[allow(dead_code)]
    pub fn ino(&self) -> u64 {
        self.ino
    }

    pub fn file_name_os_str(&self) -> &OsStr {
        self.name.as_os_str()
    }
}
```

## Block 31
**Metadata**: AST_ID=31 | TYPE=FUNCTION | NAME=new | COMPLEXITY=38 | LINES=72

```rust
impl OpenOptions {
    pub fn new() -> OpenOptions {
        OpenOptions {
            // generic
            read: false,
            write: false,
            append: false,
            truncate: false,
            create: false,
            create_new: false,
            // system-specific
            mode: 0o777,
        }
    }

    pub fn read(&mut self, read: bool) {
        self.read = read;
    }
    pub fn write(&mut self, write: bool) {
        self.write = write;
    }
    pub fn append(&mut self, append: bool) {
        self.append = append;
    }
    pub fn truncate(&mut self, truncate: bool) {
        self.truncate = truncate;
    }
    pub fn create(&mut self, create: bool) {
        self.create = create;
    }
    pub fn create_new(&mut self, create_new: bool) {
        self.create_new = create_new;
    }

    fn get_access_mode(&self) -> io::Result<i32> {
        match (self.read, self.write, self.append) {
            (true, false, false) => Ok(O_RDONLY),
            (false, true, false) => Ok(O_WRONLY),
            (true, true, false) => Ok(O_RDWR),
            (false, _, true) => Ok(O_WRONLY | O_APPEND),
            (true, _, true) => Ok(O_RDWR | O_APPEND),
            (false, false, false) => {
                Err(io::const_error!(ErrorKind::InvalidInput, "invalid access mode"))
            }
        }
    }

    fn get_creation_mode(&self) -> io::Result<i32> {
        match (self.write, self.append) {
            (true, false) => {}
            (false, false) => {
                if self.truncate || self.create || self.create_new {
                    return Err(io::const_error!(ErrorKind::InvalidInput, "invalid creation mode"));
                }
            }
            (_, true) => {
                if self.truncate && !self.create_new {
                    return Err(io::const_error!(ErrorKind::InvalidInput, "invalid creation mode"));
                }
            }
        }

        Ok(match (self.create, self.truncate, self.create_new) {
            (false, false, false) => 0,
            (true, false, false) => O_CREAT,
            (false, true, false) => O_TRUNC,
            (true, true, false) => O_CREAT | O_TRUNC,
            (_, _, true) => O_CREAT | O_EXCL,
        })
    }
}
```

## Block 32
**Metadata**: AST_ID=32 | TYPE=FUNCTION | NAME=open | COMPLEXITY=51 | LINES=118

```rust
impl File {
    pub fn open(path: &Path, opts: &OpenOptions) -> io::Result<File> {
        run_path_with_cstr(path, &|path| File::open_c(&path, opts))
    }

    pub fn open_c(path: &CStr, opts: &OpenOptions) -> io::Result<File> {
        let mut flags = opts.get_access_mode()?;
        flags = flags | opts.get_creation_mode()?;

        let mode;
        if flags & O_CREAT == O_CREAT {
            mode = opts.mode;
        } else {
            mode = 0;
        }

        let fd = unsafe { cvt(hermit_abi::open(path.as_ptr(), flags, mode))? };
        Ok(File(unsafe { FileDesc::from_raw_fd(fd as i32) }))
    }

    pub fn file_attr(&self) -> io::Result<FileAttr> {
        let mut stat_val: stat_struct = unsafe { mem::zeroed() };
        self.0.fstat(&mut stat_val)?;
        Ok(FileAttr::from_stat(stat_val))
    }

    pub fn fsync(&self) -> io::Result<()> {
        Err(Error::from_raw_os_error(22))
    }

    pub fn datasync(&self) -> io::Result<()> {
        self.fsync()
    }

    pub fn lock(&self) -> io::Result<()> {
        unsupported()
    }

    pub fn lock_shared(&self) -> io::Result<()> {
        unsupported()
    }

    pub fn try_lock(&self) -> Result<(), TryLockError> {
        Err(TryLockError::Error(unsupported_err()))
    }

    pub fn try_lock_shared(&self) -> Result<(), TryLockError> {
        Err(TryLockError::Error(unsupported_err()))
    }

    pub fn unlock(&self) -> io::Result<()> {
        unsupported()
    }

    pub fn truncate(&self, _size: u64) -> io::Result<()> {
        Err(Error::from_raw_os_error(22))
    }

    pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.read(buf)
    }

    pub fn read_vectored(&self, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
        self.0.read_vectored(bufs)
    }

    #[inline]
    pub fn is_read_vectored(&self) -> bool {
        self.0.is_read_vectored()
    }

    pub fn read_buf(&self, cursor: BorrowedCursor<'_>) -> io::Result<()> {
        self.0.read_buf(cursor)
    }

    pub fn write(&self, buf: &[u8]) -> io::Result<usize> {
        self.0.write(buf)
    }

    pub fn write_vectored(&self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
        self.0.write_vectored(bufs)
    }

    #[inline]
    pub fn is_write_vectored(&self) -> bool {
        self.0.is_write_vectored()
    }

    #[inline]
    pub fn flush(&self) -> io::Result<()> {
        Ok(())
    }

    pub fn seek(&self, pos: SeekFrom) -> io::Result<u64> {
        self.0.seek(pos)
    }

    pub fn size(&self) -> Option<io::Result<u64>> {
        None
    }

    pub fn tell(&self) -> io::Result<u64> {
        self.0.tell()
    }

    pub fn duplicate(&self) -> io::Result<File> {
        Err(Error::from_raw_os_error(22))
    }

    pub fn set_permissions(&self, _perm: FilePermissions) -> io::Result<()> {
        Err(Error::from_raw_os_error(22))
    }

    pub fn set_times(&self, _times: FileTimes) -> io::Result<()> {
        Err(Error::from_raw_os_error(22))
    }
}
```

## Block 33
**Metadata**: AST_ID=33 | TYPE=FUNCTION | NAME=new | COMPLEXITY=12 | LINES=17

```rust
impl DirBuilder {
    pub fn new() -> DirBuilder {
        DirBuilder { mode: 0o777 }
    }

    pub fn mkdir(&self, path: &Path) -> io::Result<()> {
        run_path_with_cstr(path, &|path| {
            cvt(unsafe { hermit_abi::mkdir(path.as_ptr().cast(), self.mode.into()) }).map(|_| ())
        })
    }

    #[allow(dead_code)]
    pub fn set_mode(&mut self, mode: u32) {
        self.mode = mode;
    }
}
```

## Block 34
**Metadata**: AST_ID=34 | TYPE=FUNCTION | NAME=as_inner | COMPLEXITY=5 | LINES=7

```rust
impl AsInner<FileDesc> for File {
    #[inline]
    fn as_inner(&self) -> &FileDesc {
        &self.0
    }
}
```

## Block 35
**Metadata**: AST_ID=35 | TYPE=FUNCTION | NAME=as_inner_mut | COMPLEXITY=5 | LINES=7

```rust
impl AsInnerMut<FileDesc> for File {
    #[inline]
    fn as_inner_mut(&mut self) -> &mut FileDesc {
        &mut self.0
    }
}
```

## Block 36
**Metadata**: AST_ID=36 | TYPE=FUNCTION | NAME=into_inner | COMPLEXITY=5 | LINES=6

```rust
impl IntoInner<FileDesc> for File {
    fn into_inner(self) -> FileDesc {
        self.0
    }
}
```

## Block 37
**Metadata**: AST_ID=37 | TYPE=FUNCTION | NAME=from_inner | COMPLEXITY=5 | LINES=6

```rust
impl FromInner<FileDesc> for File {
    fn from_inner(file_desc: FileDesc) -> Self {
        Self(file_desc)
    }
}
```

## Block 38
**Metadata**: AST_ID=38 | TYPE=FUNCTION | NAME=as_fd | COMPLEXITY=5 | LINES=6

```rust
impl AsFd for File {
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.0.as_fd()
    }
}
```

## Block 39
**Metadata**: AST_ID=39 | TYPE=FUNCTION | NAME=as_raw_fd | COMPLEXITY=5 | LINES=7

```rust
impl AsRawFd for File {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.0.as_raw_fd()
    }
}
```

## Block 40
**Metadata**: AST_ID=40 | TYPE=FUNCTION | NAME=into_raw_fd | COMPLEXITY=5 | LINES=6

```rust
impl IntoRawFd for File {
    fn into_raw_fd(self) -> RawFd {
        self.0.into_raw_fd()
    }
}
```

## Block 41
**Metadata**: AST_ID=41 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=14 | LINES=7

```rust
impl FromRawFd for File {
    unsafe fn from_raw_fd(raw_fd: RawFd) -> Self {
        let file_desc = unsafe { FileDesc::from_raw_fd(raw_fd) };
        Self(file_desc)
    }
}
```

## Block 42
**Metadata**: AST_ID=42 | TYPE=FUNCTION | NAME=readdir | COMPLEXITY=36 | LINES=42

```rust
pub fn readdir(path: &Path) -> io::Result<ReadDir> {
    let fd_raw = run_path_with_cstr(path, &|path| {
        cvt(unsafe { hermit_abi::open(path.as_ptr(), O_RDONLY | O_DIRECTORY, 0) })
    })?;
    let fd = unsafe { FileDesc::from_raw_fd(fd_raw as i32) };
    let root = path.to_path_buf();

    // read all director entries
    let mut vec: Vec<u8> = Vec::new();
    let mut sz = 512;
    loop {
        // reserve memory to receive all directory entries
        vec.resize(sz, 0);

        let readlen = unsafe {
            hermit_abi::getdents64(fd.as_raw_fd(), vec.as_mut_ptr() as *mut dirent64, sz)
        };
        if readlen > 0 {
            // shrink down to the minimal size
            vec.resize(readlen.try_into().unwrap(), 0);
            break;
        }

        // if the buffer is too small, getdents64 returns EINVAL
        // otherwise, getdents64 returns an error number
        if readlen != (-hermit_abi::errno::EINVAL).into() {
            return Err(Error::from_raw_os_error(readlen.try_into().unwrap()));
        }

        // we don't have enough memory => try to increase the vector size
        sz = sz * 2;

        // 1 MB for directory entries should be enough
        // stop here to avoid an endless loop
        if sz > 0x100000 {
            return Err(Error::from(ErrorKind::Uncategorized));
        }
    }

    Ok(ReadDir::new(InnerReadDir::new(root, vec)))
}
```

## Block 43
**Metadata**: AST_ID=43 | TYPE=FUNCTION | NAME=unlink | COMPLEXITY=7 | LINES=4

```rust
pub fn unlink(path: &Path) -> io::Result<()> {
    run_path_with_cstr(path, &|path| cvt(unsafe { hermit_abi::unlink(path.as_ptr()) }).map(|_| ()))
}
```

## Block 44
**Metadata**: AST_ID=44 | TYPE=FUNCTION | NAME=rename | COMPLEXITY=2 | LINES=4

```rust
pub fn rename(_old: &Path, _new: &Path) -> io::Result<()> {
    unsupported()
}
```

## Block 45
**Metadata**: AST_ID=45 | TYPE=FUNCTION | NAME=set_perm | COMPLEXITY=2 | LINES=4

```rust
pub fn set_perm(_p: &Path, _perm: FilePermissions) -> io::Result<()> {
    Err(Error::from_raw_os_error(22))
}
```

## Block 46
**Metadata**: AST_ID=46 | TYPE=FUNCTION | NAME=rmdir | COMPLEXITY=7 | LINES=4

```rust
pub fn rmdir(path: &Path) -> io::Result<()> {
    run_path_with_cstr(path, &|path| cvt(unsafe { hermit_abi::rmdir(path.as_ptr()) }).map(|_| ()))
}
```

## Block 47
**Metadata**: AST_ID=47 | TYPE=FUNCTION | NAME=remove_dir_all | COMPLEXITY=2 | LINES=4

```rust
pub fn remove_dir_all(_path: &Path) -> io::Result<()> {
    unsupported()
}
```

## Block 48
**Metadata**: AST_ID=48 | TYPE=FUNCTION | NAME=readlink | COMPLEXITY=2 | LINES=4

```rust
pub fn readlink(_p: &Path) -> io::Result<PathBuf> {
    unsupported()
}
```

## Block 49
**Metadata**: AST_ID=49 | TYPE=FUNCTION | NAME=symlink | COMPLEXITY=2 | LINES=4

```rust
pub fn symlink(_original: &Path, _link: &Path) -> io::Result<()> {
    unsupported()
}
```

## Block 50
**Metadata**: AST_ID=50 | TYPE=FUNCTION | NAME=link | COMPLEXITY=2 | LINES=4

```rust
pub fn link(_original: &Path, _link: &Path) -> io::Result<()> {
    unsupported()
}
```

## Block 51
**Metadata**: AST_ID=51 | TYPE=FUNCTION | NAME=stat | COMPLEXITY=13 | LINES=8

```rust
pub fn stat(path: &Path) -> io::Result<FileAttr> {
    run_path_with_cstr(path, &|path| {
        let mut stat_val: stat_struct = unsafe { mem::zeroed() };
        cvt(unsafe { hermit_abi::stat(path.as_ptr(), &mut stat_val) })?;
        Ok(FileAttr::from_stat(stat_val))
    })
}
```

## Block 52
**Metadata**: AST_ID=52 | TYPE=FUNCTION | NAME=lstat | COMPLEXITY=13 | LINES=8

```rust
pub fn lstat(path: &Path) -> io::Result<FileAttr> {
    run_path_with_cstr(path, &|path| {
        let mut stat_val: stat_struct = unsafe { mem::zeroed() };
        cvt(unsafe { hermit_abi::lstat(path.as_ptr(), &mut stat_val) })?;
        Ok(FileAttr::from_stat(stat_val))
    })
}
```

## Block 53
**Metadata**: AST_ID=53 | TYPE=FUNCTION | NAME=canonicalize | COMPLEXITY=2 | LINES=4

```rust
pub fn canonicalize(_p: &Path) -> io::Result<PathBuf> {
    unsupported()
}
```

---
*Generated by AST tracing system*
