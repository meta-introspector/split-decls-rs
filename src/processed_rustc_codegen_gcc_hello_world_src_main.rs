// SRC: ../rust/compiler/rustc_codegen_gcc/tests/hello-world/src/main.rs
/* AST_META: AST_ID=1 | TYPE=FUNCTION | NAME=main | COMPLEXITY=3 | LINES=5 */
use mylib::my_func;

fn main() {
    println!("{}", my_func(5, 10));
}