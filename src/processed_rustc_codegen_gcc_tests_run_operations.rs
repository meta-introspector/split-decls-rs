// SRC: ../rust/compiler/rustc_codegen_gcc/tests/run/operations.rs
/* AST_META: AST_ID=1 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=7 | LINES=23 */
// Compiler:
//
// Run-time:
//   stdout: 41
//     39
//     10

#[feature(no_core)]
#[no_std]
#[no_core]
#[no_main]

use mini_core::*;

#[unsafe(no_mangle)]
extern "C" fn main(argc: i32, _argv: *const *const u8) -> i32 {
    unsafe {
        libc::printf(b"%ld\n\0" as *const u8 as *const i8, 40 + argc);
        libc::printf(b"%ld\n\0" as *const u8 as *const i8, 40 - argc);
        libc::printf(b"%ld\n\0" as *const u8 as *const i8, 10 * argc);
    }
    0
}