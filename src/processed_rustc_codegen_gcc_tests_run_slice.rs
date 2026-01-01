// SRC: ../rust/compiler/rustc_codegen_gcc/tests/run/slice.rs
/* AST_META: AST_ID=1 | TYPE=FUNCTION | NAME=index_slice | COMPLEXITY=7 | LINES=18 */
// Compiler:
//
// Run-time:
//   status: 0
//   stdout: 5

#[feature(no_core)]
#[no_std]
#[no_core]
#[no_main]

use mini_core::*;

static mut TWO: usize = 2;

fn index_slice(s: &[u32]) -> u32 {
    unsafe { s[TWO] }
}
/* AST_META: AST_ID=2 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=7 | LINES=9 */

#[unsafe(no_mangle)]
extern "C" fn main(argc: i32, _argv: *const *const u8) -> i32 {
    let array = [42, 7, 5];
    unsafe {
        libc::printf(b"%ld\n\0" as *const u8 as *const i8, index_slice(&array));
    }
    0
}