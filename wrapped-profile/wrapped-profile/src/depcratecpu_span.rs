// Generated macro for cpu_span (function)
macro_rules! Depcratecpu_span {
() => {
// Module: crate
// Provides: {"cpu_span"}
// Dependencies: {}
# [must_use] pub fn cpu_span () -> CpuSpan { # [cfg (feature = "cpu_profiler")] { google_cpu_profiler :: start ("./out.profile" . as_ref ()) } # [cfg (not (feature = "cpu_profiler"))] # [allow (clippy :: print_stderr)] { eprintln ! (r#"cpu profiling is disabled, uncomment `default = [ "cpu_profiler" ]` in Cargo.toml to enable."#) ; } CpuSpan { _private : () } }
};
}
