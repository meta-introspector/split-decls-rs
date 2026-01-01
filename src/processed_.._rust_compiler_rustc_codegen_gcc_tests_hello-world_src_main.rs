// SRC: ../rust/compiler/rustc_codegen_gcc/tests/hello-world/src/main.rs
use mylib::my_func;

fn main() {
    println!("{}", my_func(5, 10));
}
