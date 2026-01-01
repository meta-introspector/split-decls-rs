// SRC: ../rust/library/std/src/sys/pal/solid/abi/fs.rs
/* AST_META: AST_ID=1 | TYPE=STATEMENT | COMPLEXITY=1 */
//! `solid_fs.h`

/* AST_META: AST_ID=2 | TYPE=STATEMENT | COMPLEXITY=2 */
pub use libc::{
/* AST_META: AST_ID=3 | TYPE=STATEMENT | COMPLEXITY=2 */
    O_APPEND, O_CREAT, O_EXCL, O_RDONLY, O_RDWR, O_TRUNC, O_WRONLY, S_IFBLK, S_IFCHR, S_IFDIR,
/* AST_META: AST_ID=4 | TYPE=STATEMENT | COMPLEXITY=2 */
    S_IFIFO, S_IFMT, S_IFREG, S_IWRITE, SEEK_CUR, SEEK_END, SEEK_SET, ino_t, off_t, stat, time_t,
/* AST_META: AST_ID=5 | TYPE=STATEMENT | COMPLEXITY=1 */
};

/* AST_META: AST_ID=6 | TYPE=USE | COMPLEXITY=2 */
use crate::os::raw::{c_char, c_int, c_uchar};

/* AST_META: AST_ID=7 | TYPE=STATEMENT | COMPLEXITY=1 */
pub const O_ACCMODE: c_int = 0x3;

/* AST_META: AST_ID=8 | TYPE=STATEMENT | COMPLEXITY=1 */
pub const SOLID_MAX_PATH: usize = 256;

/* AST_META: AST_ID=9 | TYPE=STATEMENT | COMPLEXITY=1 */
#[repr(C)]
/* AST_META: AST_ID=10 | TYPE=STATEMENT | COMPLEXITY=1 */
#[derive(Copy, Clone)]
/* AST_META: AST_ID=11 | TYPE=STATEMENT | COMPLEXITY=2 */
pub struct dirent {
/* AST_META: AST_ID=12 | TYPE=STATEMENT | COMPLEXITY=1 */
    pub d_ino: ino_t,
/* AST_META: AST_ID=13 | TYPE=STATEMENT | COMPLEXITY=1 */
    pub d_type: c_uchar,
/* AST_META: AST_ID=14 | TYPE=STATEMENT | COMPLEXITY=1 */
    pub d_name: [c_char; 256usize],
/* AST_META: AST_ID=15 | TYPE=STATEMENT | COMPLEXITY=1 */
}

/* AST_META: AST_ID=16 | TYPE=STATEMENT | COMPLEXITY=1 */
pub const DT_UNKNOWN: c_uchar = 0;
/* AST_META: AST_ID=17 | TYPE=STATEMENT | COMPLEXITY=1 */
pub const DT_FIFO: c_uchar = 1;
/* AST_META: AST_ID=18 | TYPE=STATEMENT | COMPLEXITY=1 */
pub const DT_CHR: c_uchar = 2;
/* AST_META: AST_ID=19 | TYPE=STATEMENT | COMPLEXITY=1 */
pub const DT_DIR: c_uchar = 4;
/* AST_META: AST_ID=20 | TYPE=STATEMENT | COMPLEXITY=1 */
pub const DT_BLK: c_uchar = 6;
/* AST_META: AST_ID=21 | TYPE=STATEMENT | COMPLEXITY=1 */
pub const DT_REG: c_uchar = 8;
/* AST_META: AST_ID=22 | TYPE=STATEMENT | COMPLEXITY=1 */
pub const DT_LNK: c_uchar = 10;
/* AST_META: AST_ID=23 | TYPE=STATEMENT | COMPLEXITY=1 */
pub const DT_SOCK: c_uchar = 12;
/* AST_META: AST_ID=24 | TYPE=STATEMENT | COMPLEXITY=1 */
pub const DT_WHT: c_uchar = 14;

/* AST_META: AST_ID=25 | TYPE=STATEMENT | COMPLEXITY=1 */
pub type S_DIR = c_int;

/* AST_META: AST_ID=26 | TYPE=STATEMENT | COMPLEXITY=2 */
unsafe extern "C" {
/* AST_META: AST_ID=27 | TYPE=STATEMENT | COMPLEXITY=2 */
    pub fn SOLID_FS_Open(fd: *mut c_int, path: *const c_char, mode: c_int) -> c_int;
/* AST_META: AST_ID=28 | TYPE=STATEMENT | COMPLEXITY=1 */
    pub fn SOLID_FS_Close(fd: c_int) -> c_int;
/* AST_META: AST_ID=29 | TYPE=STATEMENT | COMPLEXITY=2 */
    pub fn SOLID_FS_Read(fd: c_int, buf: *mut u8, size: usize, result: *mut usize) -> c_int;
/* AST_META: AST_ID=30 | TYPE=STATEMENT | COMPLEXITY=2 */
    pub fn SOLID_FS_Write(fd: c_int, buf: *const u8, size: usize, result: *mut usize) -> c_int;
/* AST_META: AST_ID=31 | TYPE=STATEMENT | COMPLEXITY=2 */
    pub fn SOLID_FS_Lseek(fd: c_int, offset: off_t, whence: c_int) -> c_int;
/* AST_META: AST_ID=32 | TYPE=STATEMENT | COMPLEXITY=1 */
    pub fn SOLID_FS_Sync(fd: c_int) -> c_int;
/* AST_META: AST_ID=33 | TYPE=STATEMENT | COMPLEXITY=2 */
    pub fn SOLID_FS_Ftell(fd: c_int, result: *mut off_t) -> c_int;
/* AST_META: AST_ID=34 | TYPE=STATEMENT | COMPLEXITY=2 */
    pub fn SOLID_FS_Feof(fd: c_int, result: *mut c_int) -> c_int;
/* AST_META: AST_ID=35 | TYPE=STATEMENT | COMPLEXITY=2 */
    pub fn SOLID_FS_Fsize(fd: c_int, result: *mut usize) -> c_int;
/* AST_META: AST_ID=36 | TYPE=STATEMENT | COMPLEXITY=2 */
    pub fn SOLID_FS_Truncate(path: *const c_char, size: off_t) -> c_int;
/* AST_META: AST_ID=37 | TYPE=STATEMENT | COMPLEXITY=2 */
    pub fn SOLID_FS_OpenDir(path: *const c_char, pDir: *mut S_DIR) -> c_int;
/* AST_META: AST_ID=38 | TYPE=STATEMENT | COMPLEXITY=2 */
    pub fn SOLID_FS_CloseDir(dir: S_DIR) -> c_int;
/* AST_META: AST_ID=39 | TYPE=STATEMENT | COMPLEXITY=2 */
    pub fn SOLID_FS_ReadDir(dir: S_DIR, dirp: *mut dirent) -> c_int;
/* AST_META: AST_ID=40 | TYPE=STATEMENT | COMPLEXITY=2 */
    pub fn SOLID_FS_Stat(path: *const c_char, buf: *mut stat) -> c_int;
/* AST_META: AST_ID=41 | TYPE=STATEMENT | COMPLEXITY=2 */
    pub fn SOLID_FS_Unlink(path: *const c_char) -> c_int;
/* AST_META: AST_ID=42 | TYPE=STATEMENT | COMPLEXITY=2 */
    pub fn SOLID_FS_Rename(oldpath: *const c_char, newpath: *const c_char) -> c_int;
/* AST_META: AST_ID=43 | TYPE=STATEMENT | COMPLEXITY=2 */
    pub fn SOLID_FS_Chmod(path: *const c_char, mode: c_int) -> c_int;
/* AST_META: AST_ID=44 | TYPE=STATEMENT | COMPLEXITY=2 */
    pub fn SOLID_FS_Utime(path: *const c_char, time: time_t) -> c_int;
/* AST_META: AST_ID=45 | TYPE=STATEMENT | COMPLEXITY=2 */
    pub fn SOLID_FS_Mkdir(path: *const c_char) -> c_int;
/* AST_META: AST_ID=46 | TYPE=STATEMENT | COMPLEXITY=1 */
}
