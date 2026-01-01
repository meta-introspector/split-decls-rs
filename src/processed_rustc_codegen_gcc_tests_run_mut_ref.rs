// SRC: ../rust/compiler/rustc_codegen_gcc/tests/run/mut_ref.rs
/* AST_META: AST_ID=1 | TYPE=STRUCT | NAME=Test | COMPLEXITY=2 | LINES=18 */
// Compiler:
//
// Run-time:
//   stdout: 2
//     7
//     6
//     11

#[feature(no_core)]
#[no_std]
#[no_core]
#[no_main]

use mini_core::*;

struct Test {
    field: isize,
}
/* AST_META: AST_ID=2 | TYPE=FUNCTION | NAME=test | COMPLEXITY=3 | LINES=4 */

fn test(num: isize) -> Test {
    Test { field: num + 1 }
}
/* AST_META: AST_ID=3 | TYPE=FUNCTION | NAME=update_num | COMPLEXITY=2 | LINES=4 */

fn update_num(num: &mut isize) {
    *num = *num + 5;
}
/* AST_META: AST_ID=4 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=23 | LINES=25 */

#[unsafe(no_mangle)]
extern "C" fn main(mut argc: isize, _argv: *const *const u8) -> i32 {
    let mut test = test(argc);
    unsafe {
        libc::printf(b"%ld\n\0" as *const u8 as *const i8, test.field);
    }
    update_num(&mut test.field);
    unsafe {
        libc::printf(b"%ld\n\0" as *const u8 as *const i8, test.field);
    }

    update_num(&mut argc);
    unsafe {
        libc::printf(b"%ld\n\0" as *const u8 as *const i8, argc);
    }

    let refe = &mut argc;
    *refe = *refe + 5;
    unsafe {
        libc::printf(b"%ld\n\0" as *const u8 as *const i8, argc);
    }

    0
}