// SRC: ../rust/compiler/rustc_codegen_gcc/tests/run/abort2.rs
/* AST_META: AST_ID=1 | TYPE=FUNCTION | NAME=fail | COMPLEXITY=7 | LINES=16 */
// Compiler:
//
// Run-time:
//   status: signal

#[feature(no_core)]
#[no_std]
#[no_core]
#[no_main]

use mini_core::*;

fn fail() -> i32 {
    unsafe { intrinsics::abort() };
    0
}
/* AST_META: AST_ID=2 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=2 | LINES=6 */

#[unsafe(no_mangle)]
extern "C" fn main(argc: i32, _argv: *const *const u8) -> i32 {
    fail();
    0
}