// SRC: ../rust/compiler/rustc_codegen_gcc/tests/run/abort1.rs
/* AST_META: AST_ID=1 | TYPE=FUNCTION | NAME=test_fail | COMPLEXITY=7 | LINES=15 */
// Compiler:
//
// Run-time:
//   status: signal

#[feature(no_core)]
#[no_std]
#[no_core]
#[no_main]

use mini_core::*;

fn test_fail() -> ! {
    unsafe { intrinsics::abort() };
}
/* AST_META: AST_ID=2 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=2 | LINES=5 */

#[unsafe(no_mangle)]
extern "C" fn main(argc: i32, _argv: *const *const u8) -> i32 {
    test_fail();
}