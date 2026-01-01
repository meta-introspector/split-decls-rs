// SRC: ../rust/compiler/rustc_codegen_llvm/src/value.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1 */
use std::hash::{Hash, Hasher};
/* AST_META: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1 */
use std::{fmt, ptr};
/* AST_META: AST_ID=3 | TYPE=FUNCTION | NAME=eq | COMPLEXITY=5 | LINES=9 */

use crate::llvm;
pub(crate) use crate::llvm::Value;

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        ptr::eq(self, other)
    }
}
/* AST_META: AST_ID=4 | TYPE=IMPL | NAME=UNNAMED | COMPLEXITY=4 | LINES=2 */

impl Eq for Value {}
/* AST_META: AST_ID=5 | TYPE=FUNCTION | NAME=hash | COMPLEXITY=5 | LINES=6 */

impl Hash for Value {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        (self as *const Self).hash(hasher);
    }
}
/* AST_META: AST_ID=6 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=10 | LINES=11 */

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(
            &llvm::build_string(|s| unsafe {
                llvm::LLVMRustWriteValueToString(self, s);
            })
            .expect("non-UTF8 value description from LLVM"),
        )
    }
}