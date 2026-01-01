// SRC: ../rust/compiler/rustc_data_structures/src/flock/unix.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1 */
use std::fs::{File, OpenOptions};
/* AST_META: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3 */
use std::os::unix::prelude::*;
use std::path::Path;
use std::{io, mem};
/* AST_META: AST_ID=3 | TYPE=STRUCT | NAME=Lock | COMPLEXITY=2 | LINES=5 */

#[derive(Debug)]
pub struct Lock {
    file: File,
}
/* AST_META: AST_ID=4 | TYPE=FUNCTION | NAME=new | COMPLEXITY=31 | LINES=35 */

impl Lock {
    pub fn new(p: &Path, wait: bool, create: bool, exclusive: bool) -> io::Result<Lock> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(create)
            .mode(libc::S_IRWXU as u32)
            .open(p)?;

        let lock_type = if exclusive { libc::F_WRLCK } else { libc::F_RDLCK };

        let mut flock: libc::flock = unsafe { mem::zeroed() };
        #[cfg(not(all(target_os = "hurd", target_arch = "x86")))]
        {
            flock.l_type = lock_type as libc::c_short;
            flock.l_whence = libc::SEEK_SET as libc::c_short;
        }
        #[cfg(all(target_os = "hurd", target_arch = "x86"))]
        {
            flock.l_type = lock_type as libc::c_int;
            flock.l_whence = libc::SEEK_SET as libc::c_int;
        }
        flock.l_start = 0;
        flock.l_len = 0;

        let cmd = if wait { libc::F_SETLKW } else { libc::F_SETLK };
        let ret = unsafe { libc::fcntl(file.as_raw_fd(), cmd, &flock) };
        if ret == -1 { Err(io::Error::last_os_error()) } else { Ok(Lock { file }) }
    }

    pub fn error_unsupported(err: &io::Error) -> bool {
        matches!(err.raw_os_error(), Some(libc::ENOTSUP) | Some(libc::ENOSYS))
    }
}
/* AST_META: AST_ID=5 | TYPE=FUNCTION | NAME=drop | COMPLEXITY=18 | LINES=22 */

impl Drop for Lock {
    fn drop(&mut self) {
        let mut flock: libc::flock = unsafe { mem::zeroed() };
        #[cfg(not(all(target_os = "hurd", target_arch = "x86")))]
        {
            flock.l_type = libc::F_UNLCK as libc::c_short;
            flock.l_whence = libc::SEEK_SET as libc::c_short;
        }
        #[cfg(all(target_os = "hurd", target_arch = "x86"))]
        {
            flock.l_type = libc::F_UNLCK as libc::c_int;
            flock.l_whence = libc::SEEK_SET as libc::c_int;
        }
        flock.l_start = 0;
        flock.l_len = 0;

        unsafe {
            libc::fcntl(self.file.as_raw_fd(), libc::F_SETLK, &flock);
        }
    }
}