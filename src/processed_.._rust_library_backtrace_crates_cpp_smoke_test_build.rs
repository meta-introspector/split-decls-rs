// SRC: ../rust/library/backtrace/crates/cpp_smoke_test/build.rs
fn main() {
    compile_cpp();
}

fn compile_cpp() {
    println!("cargo:rerun-if-changed=cpp/trampoline.cpp");

    cc::Build::new()
        .cpp(true)
        .debug(true)
        .opt_level(0)
        .file("cpp/trampoline.cpp")
        .compile("libcpptrampoline.a");
}
