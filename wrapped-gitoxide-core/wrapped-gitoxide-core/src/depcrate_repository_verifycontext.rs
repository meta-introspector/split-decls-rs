// Generated macro for Context (struct)
macro_rules! Depcrate_repository_verifyContext {
() => {
// Module: crate::repository::verify
// Provides: {"Context"}
// Dependencies: {}
# [doc = " A general purpose context for many operations provided here"] pub struct Context { # [doc = " If set, provide statistics to `out` in the given format"] pub output_statistics : Option < OutputFormat > , # [doc = " If set, don't use more than this amount of threads."] # [doc = " Otherwise, usually use as many threads as there are logical cores."] # [doc = " A value of 0 is interpreted as no-limit"] pub thread_limit : Option < usize > , pub verify_mode : pack :: verify :: Mode , pub algorithm : pack :: verify :: Algorithm , }
};
}
