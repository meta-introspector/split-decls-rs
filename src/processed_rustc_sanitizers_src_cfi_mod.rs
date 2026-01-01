// SRC: ../rust/compiler/rustc_sanitizers/src/cfi/mod.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=6 | LINES=6 */
// LLVM Control Flow Integrity (CFI) and cross-language LLVM CFI support for the Rust compiler.
//
// For more information about LLVM CFI and cross-language LLVM CFI support for the Rust compiler,
// see design document in the tracking issue #89653.
pub use crate::cfi::typeid::{TypeIdOptions, typeid_for_fnabi, typeid_for_instance};