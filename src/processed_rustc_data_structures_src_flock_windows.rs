// SRC: ../rust/compiler/rustc_data_structures/src/flock/windows.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1 */
use std::fs::{File, OpenOptions};
/* AST_META: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=6 */
use std::io;
use std::os::windows::prelude::*;
use std::path::Path;

use tracing::debug;
use windows::Win32::Foundation::{ERROR_INVALID_FUNCTION, HANDLE};
/* AST_META: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=4 */
use windows::Win32::Storage::FileSystem::{
    FILE_SHARE_DELETE, FILE_SHARE_READ, FILE_SHARE_WRITE, LOCK_FILE_FLAGS, LOCKFILE_EXCLUSIVE_LOCK,
    LOCKFILE_FAIL_IMMEDIATELY, LockFileEx,
};
/* AST_META: AST_ID=4 | TYPE=STRUCT | NAME=Lock | COMPLEXITY=2 | LINES=6 */
use windows::Win32::System::IO::OVERLAPPED;

#[derive(Debug)]
pub struct Lock {
    _file: File,
}
/* AST_META: AST_ID=5 | TYPE=FUNCTION | NAME=new | COMPLEXITY=34 | LINES=67 */

impl Lock {
    pub fn new(p: &Path, wait: bool, create: bool, exclusive: bool) -> io::Result<Lock> {
        assert!(
            p.parent().unwrap().exists(),
            "Parent directory of lock-file must exist: {}",
            p.display()
        );

        let share_mode = FILE_SHARE_DELETE | FILE_SHARE_READ | FILE_SHARE_WRITE;

        let mut open_options = OpenOptions::new();
        open_options.read(true).share_mode(share_mode.0);

        if create {
            open_options.create(true).write(true);
        }

        debug!("attempting to open lock file `{}`", p.display());
        let file = match open_options.open(p) {
            Ok(file) => {
                debug!("lock file opened successfully");
                file
            }
            Err(err) => {
                debug!("error opening lock file: {}", err);
                return Err(err);
            }
        };

        let mut flags = LOCK_FILE_FLAGS::default();
        if !wait {
            flags |= LOCKFILE_FAIL_IMMEDIATELY;
        }

        if exclusive {
            flags |= LOCKFILE_EXCLUSIVE_LOCK;
        }

        let mut overlapped = OVERLAPPED::default();

        debug!("attempting to acquire lock on lock file `{}`", p.display());

        unsafe {
            LockFileEx(
                HANDLE(file.as_raw_handle()),
                flags,
                None,
                u32::MAX,
                u32::MAX,
                &mut overlapped,
            )
        }
        .map_err(|e| {
            let err = io::Error::from_raw_os_error(e.code().0);
            debug!("failed acquiring file lock: {}", err);
            err
        })?;

        debug!("successfully acquired lock");
        Ok(Lock { _file: file })
    }

    pub fn error_unsupported(err: &io::Error) -> bool {
        err.raw_os_error() == Some(ERROR_INVALID_FUNCTION.0 as i32)
    }
}
/* AST_META: AST_ID=6 | TYPE=IMPL | NAME=UNNAMED | COMPLEXITY=1 | LINES=3 */

// Note that we don't need a Drop impl on Windows: The file is unlocked
// automatically when it's closed.