// SRC: ../rust/compiler/rustc_codegen_gcc/tests/run/static.rs
/* AST_META: AST_ID=1 | TYPE=STRUCT | NAME=Test | COMPLEXITY=2 | LINES=21 */
// Compiler:
//
// Run-time:
//   status: 0
//   stdout: 10
//      14
//      1
//      12
//      12
//      1

#[feature(no_core)]
#[no_std]
#[no_core]
#[no_main]

use mini_core::*;

struct Test {
    field: isize,
}
/* AST_META: AST_ID=2 | TYPE=STRUCT | NAME=WithRef | COMPLEXITY=2 | LINES=4 */

struct WithRef {
    refe: &'static Test,
}
/* AST_META: AST_ID=3 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=4 */

static mut CONSTANT: isize = 10;

static mut TEST: Test = Test { field: 12 };
/* AST_META: AST_ID=4 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=2 */

static mut TEST2: Test = Test { field: 14 };
/* AST_META: AST_ID=5 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=7 | LINES=2 */

static mut WITH_REF: WithRef = WithRef { refe: unsafe { &TEST } };
/* AST_META: AST_ID=6 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=8 | LINES=15 */

#[unsafe(no_mangle)]
extern "C" fn main(argc: isize, _argv: *const *const u8) -> i32 {
    unsafe {
        libc::printf(b"%ld\n\0" as *const u8 as *const i8, CONSTANT);
        libc::printf(b"%ld\n\0" as *const u8 as *const i8, TEST2.field);
        TEST2.field = argc;
        libc::printf(b"%ld\n\0" as *const u8 as *const i8, TEST2.field);
        libc::printf(b"%ld\n\0" as *const u8 as *const i8, WITH_REF.refe.field);
        WITH_REF.refe = &TEST2;
        libc::printf(b"%ld\n\0" as *const u8 as *const i8, TEST.field);
        libc::printf(b"%ld\n\0" as *const u8 as *const i8, WITH_REF.refe.field);
    }
    0
}