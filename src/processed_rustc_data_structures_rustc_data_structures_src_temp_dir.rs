// SRC: ../rust/compiler/rustc_data_structures/src/temp_dir.rs
/* AST_META: AST_ID=1 | TYPE=STRUCT | NAME=MaybeTempDir | COMPLEXITY=2 | LINES=12 */
use std::mem::ManuallyDrop;
use std::path::Path;

use tempfile::TempDir;

/// This is used to avoid TempDir being dropped on error paths unintentionally.
#[derive(Debug)]
pub struct MaybeTempDir {
    dir: ManuallyDrop<TempDir>,
    // Whether the TempDir should be deleted on drop.
    keep: bool,
}
/* AST_META: AST_ID=2 | TYPE=FUNCTION | NAME=drop | COMPLEXITY=13 | LINES=11 */

impl Drop for MaybeTempDir {
    fn drop(&mut self) {
        // SAFETY: We are in the destructor, and no further access will
        // occur.
        let dir = unsafe { ManuallyDrop::take(&mut self.dir) };
        if self.keep {
            let _ = dir.keep();
        }
    }
}
/* AST_META: AST_ID=3 | TYPE=FUNCTION | NAME=as_ref | COMPLEXITY=5 | LINES=6 */

impl AsRef<Path> for MaybeTempDir {
    fn as_ref(&self) -> &Path {
        self.dir.path()
    }
}
/* AST_META: AST_ID=4 | TYPE=FUNCTION | NAME=new | COMPLEXITY=4 | LINES=6 */

impl MaybeTempDir {
    pub fn new(dir: TempDir, keep_on_drop: bool) -> MaybeTempDir {
        MaybeTempDir { dir: ManuallyDrop::new(dir), keep: keep_on_drop }
    }
}