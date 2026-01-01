// SRC: ../rust/compiler/rustc_span/src/fatal_error.rs
/* AST_META: AST_ID=1 | TYPE=STRUCT | NAME=FatalError; | COMPLEXITY=6 | LINES=10 */
/// Used as a return value to signify a fatal error occurred.
#[derive(Copy, Clone, Debug)]
#[must_use]
pub struct FatalError;

pub use crate::rustc_data_structures::FatalErrorMarker;

// Don't implement Send on FatalError. This makes it impossible to `panic_any!(FatalError)`.
// We don't want to invoke the panic handler and print a backtrace for fatal errors.
impl !Send for FatalError {}
/* AST_META: AST_ID=2 | TYPE=FUNCTION | NAME=raise | COMPLEXITY=3 | LINES=6 */

impl FatalError {
    pub fn raise(self) -> ! {
        std::panic::resume_unwind(Box::new(FatalErrorMarker))
    }
}
/* AST_META: AST_ID=3 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=5 | LINES=6 */

impl std::fmt::Display for FatalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "fatal error")
    }
}
/* AST_META: AST_ID=4 | TYPE=IMPL | NAME=UNNAMED | COMPLEXITY=4 | LINES=2 */

impl std::error::Error for FatalError {}