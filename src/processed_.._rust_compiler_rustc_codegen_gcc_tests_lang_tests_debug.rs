// SRC: ../rust/compiler/rustc_codegen_gcc/tests/lang_tests_debug.rs
mod lang_tests_common;

fn main() {
    lang_tests_common::main_inner(lang_tests_common::Profile::Debug);
}
