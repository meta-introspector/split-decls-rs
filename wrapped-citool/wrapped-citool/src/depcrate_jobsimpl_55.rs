// Generated macro for impl_55 (impl)
macro_rules! Depcrate_jobsimpl_55 {
() => {
// Module: crate::jobs
// Provides: {"impl_55"}
// Dependencies: {}
impl Job { # [doc = " By default, the Docker image of a job is based on its name."] # [doc = " However, it can be overridden by its IMAGE environment variable."] pub fn image (& self) -> String { self . env . get ("IMAGE") . map (| v | v . as_str () . expect ("IMAGE value should be a string") . to_string ()) . unwrap_or_else (| | self . name . clone ()) } fn is_linux (& self) -> bool { self . os . contains ("ubuntu") } }
};
}
