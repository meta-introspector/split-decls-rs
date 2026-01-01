// SRC: ../rust/compiler/rustc_codegen_gcc/tests/run/empty_main.rs
/* AST_META: AST_ID=1 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=2 | LINES=16 */
// Compiler:
//
// Run-time:
//   status: 0

#[feature(no_core)]
#[no_std]
#[no_core]
#[no_main]

use mini_core::*;

#[unsafe(no_mangle)]
extern "C" fn main(argc: i32, _argv: *const *const u8) -> i32 {
    0
}