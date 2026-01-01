// SRC: ../rust/compiler/rustc_codegen_gcc/tests/run/assign.rs
/* AST_META: AST_ID=1 | TYPE=FUNCTION | NAME=inc_ref | COMPLEXITY=2 | LINES=18 */
// Compiler:
//
// Run-time:
//   stdout: 2
//     7 8
//     10

#[feature(no_core)]
#[no_std]
#[no_core]
#[no_main]

use mini_core::*;

fn inc_ref(num: &mut isize) -> isize {
    *num = *num + 5;
    *num + 1
}
/* AST_META: AST_ID=2 | TYPE=FUNCTION | NAME=inc | COMPLEXITY=2 | LINES=4 */

fn inc(num: isize) -> isize {
    num + 1
}
/* AST_META: AST_ID=3 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=17 | LINES=19 */

#[unsafe(no_mangle)]
extern "C" fn main(mut argc: isize, _argv: *const *const u8) -> i32 {
    argc = inc(argc);
    unsafe {
        libc::printf(b"%ld\n\0" as *const u8 as *const i8, argc);
    }

    let b = inc_ref(&mut argc);
    unsafe {
        libc::printf(b"%ld %ld\n\0" as *const u8 as *const i8, argc, b);
    }

    argc = 10;
    unsafe {
        libc::printf(b"%ld\n\0" as *const u8 as *const i8, argc);
    }
    0
}