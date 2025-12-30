// Generated macro for compute_clang_target (function)
macro_rules! Depcratecompute_clang_target {
() => {
// Module: crate
// Provides: {"compute_clang_target"}
// Dependencies: {}
fn compute_clang_target (rust_arch : & str) -> String { if rust_arch == "x86" { format ! ("i686-unknown-linux") } else if rust_arch == "x32" { format ! ("x86_64-unknown-linux-gnux32") } else if rust_arch == "mips32r6" { format ! ("mipsisa32r6-unknown-linux-gnu") } else if rust_arch == "mips64r6" { format ! ("mipsisa64r6-unknown-linux-gnuabi64") } else { format ! ("{}-unknown-linux" , rust_arch) } }
};
}
