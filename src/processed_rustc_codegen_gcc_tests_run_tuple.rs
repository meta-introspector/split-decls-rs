// SRC: ../rust/compiler/rustc_codegen_gcc/tests/run/tuple.rs
/* AST_META: AST_ID=1 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=7 | LINES=21 */
// Compiler:
//
// Run-time:
//   status: 0
//   stdout: 3

#[feature(no_core)]
#[no_std]
#[no_core]
#[no_main]

use mini_core::*;

#[unsafe(no_mangle)]
extern "C" fn main(argc: i32, _argv: *const *const u8) -> i32 {
    let test: (isize, isize, isize) = (3, 1, 4);
    unsafe {
        libc::printf(b"%ld\n\0" as *const u8 as *const i8, test.0);
    }
    0
}