// SRC: ../rust/compiler/rustc_data_structures/src/flock/unsupported.rs
/* AST_META: AST_ID=1 | TYPE=FUNCTION | NAME=Lock(()); | COMPLEXITY=4 | LINES=16 */
use std::io;
use std::path::Path;

#[derive(Debug)]
pub struct Lock(());

impl Lock {
    pub fn new(_p: &Path, _wait: bool, _create: bool, _exclusive: bool) -> io::Result<Lock> {
        let msg = "file locks not supported on this platform";
        Err(io::Error::new(io::ErrorKind::Other, msg))
    }

    pub fn error_unsupported(_err: &io::Error) -> bool {
        true
    }
}