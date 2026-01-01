// SRC: ../rust/compiler/rustc_codegen_gcc/tests/run/structs.rs
/* AST_META: AST_ID=1 | TYPE=STRUCT | NAME=Test | COMPLEXITY=2 | LINES=17 */
// Compiler:
//
// Run-time:
//   status: 0
//   stdout: 1
//     2

#[feature(no_core)]
#[no_std]
#[no_core]
#[no_main]

use mini_core::*;

struct Test {
    field: isize,
}
/* AST_META: AST_ID=2 | TYPE=STRUCT | NAME=Two | COMPLEXITY=2 | LINES=4 */

struct Two {
    two: isize,
}
/* AST_META: AST_ID=3 | TYPE=FUNCTION | NAME=one | COMPLEXITY=2 | LINES=4 */

fn one() -> isize {
    1
}
/* AST_META: AST_ID=4 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=9 | LINES=11 */

#[unsafe(no_mangle)]
extern "C" fn main(argc: i32, _argv: *const *const u8) -> i32 {
    let test = Test { field: one() };
    let two = Two { two: 2 };
    unsafe {
        libc::printf(b"%ld\n\0" as *const u8 as *const i8, test.field);
        libc::printf(b"%ld\n\0" as *const u8 as *const i8, two.two);
    }
    0
}