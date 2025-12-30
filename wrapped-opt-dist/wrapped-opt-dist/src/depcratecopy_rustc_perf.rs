// Generated macro for copy_rustc_perf (function)
macro_rules! Depcratecopy_rustc_perf {
() => {
// Module: crate
// Provides: {"copy_rustc_perf"}
// Dependencies: {}
fn copy_rustc_perf (env : & Environment , dir : & Utf8Path) -> anyhow :: Result < () > { copy_directory (dir , & env . rustc_perf_dir ()) ? ; build_rustc_perf (env) }
};
}
