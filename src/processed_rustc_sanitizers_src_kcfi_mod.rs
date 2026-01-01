// SRC: ../rust/compiler/rustc_sanitizers/src/kcfi/mod.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=6 | LINES=7 */
// LLVM Kernel Control Flow Integrity (KCFI) and cross-language LLVM KCFI support for the Rust
// compiler.
//
// For more information about LLVM KCFI and cross-language LLVM KCFI support for the Rust compiler,
// see the tracking issue #123479.
pub use crate::kcfi::typeid::{TypeIdOptions, typeid_for_fnabi, typeid_for_instance};