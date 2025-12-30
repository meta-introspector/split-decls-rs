// Generated macro for DhatRunner (struct)
macro_rules! Depcrate_valgrindDhatRunner {
() => {
// Module: crate::valgrind
// Provides: {"DhatRunner"}
// Dependencies: {}
# [doc = " A DHAT-based benchmark runner that measures runtime memory use."] pub (crate) struct DhatRunner { # [doc = " The path to the ci-bench executable"] # [doc = ""] # [doc = " This is necessary because the runner works by spawning child processes"] executable : String , # [doc = " The directory where the output will be stored"] output_dir : PathBuf , }
};
}
