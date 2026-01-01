// SRC: ../rust/compiler/rustc_codegen_gcc/tests/run/volatile.rs
/* AST_META: AST_ID=1 | TYPE=STRUCT | NAME=Struct | COMPLEXITY=6 | LINES=13 */
// Compiler:
//
// Run-time:
//   status: 0

use std::mem::MaybeUninit;

#[allow(dead_code)]
#[derive(Debug)]
struct Struct {
    pointer: *const (),
    func: unsafe fn(*const ()),
}
/* AST_META: AST_ID=2 | TYPE=FUNCTION | NAME=func | COMPLEXITY=2 | LINES=2 */

fn func(_ptr: *const ()) {}
/* AST_META: AST_ID=3 | TYPE=FUNCTION | NAME=main | COMPLEXITY=14 | LINES=8 */

fn main() {
    let mut x = MaybeUninit::<&Struct>::uninit();
    x.write(&Struct { pointer: std::ptr::null(), func });
    let x = unsafe { x.assume_init() };
    let value = unsafe { (x as *const Struct).read_volatile() };
    println!("{:?}", value);
}