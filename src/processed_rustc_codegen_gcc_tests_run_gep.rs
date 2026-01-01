// SRC: ../rust/compiler/rustc_codegen_gcc/tests/run/gep.rs
/* AST_META: AST_ID=1 | TYPE=FUNCTION | NAME=main | COMPLEXITY=3 | LINES=10 */
// Compiler:
//
// Run-time:
//   status: 0

fn main() {
    let mut value = (1, 1);
    let ptr = &mut value as *mut (i32, i32);
    println!("{:?}", ptr.wrapping_offset(10));
}