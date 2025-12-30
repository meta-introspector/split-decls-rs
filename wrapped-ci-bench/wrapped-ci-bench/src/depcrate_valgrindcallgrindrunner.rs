// Generated macro for CallgrindRunner (struct)
macro_rules! Depcrate_valgrindCallgrindRunner {
() => {
// Module: crate::valgrind
// Provides: {"CallgrindRunner"}
// Dependencies: {}
# [doc = " A callgrind-based benchmark runner"] pub (crate) struct CallgrindRunner { # [doc = " The path to the ci-bench executable"] # [doc = ""] # [doc = " This is necessary because the callgrind runner works by spawning child processes"] executable : String , # [doc = " The directory where the callgrind output will be stored"] output_dir : PathBuf , }
};
}
