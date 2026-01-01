// SRC: ../rust/compiler/rustc_codegen_gcc/tests/hello-world/mylib/src/lib.rs
/* AST_META: AST_ID=1 | TYPE=FUNCTION | NAME=my_func | COMPLEXITY=5 | LINES=7 */
pub fn my_func(a: i32, b: i32) -> i32 {
    let mut res = a;
    for i in a..b {
        res += i;
    }
    res
}