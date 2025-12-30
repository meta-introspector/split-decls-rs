// Generated macro for impl_29 (impl)
macro_rules! Depcrate_project_jsonimpl_29 {
() => {
// Module: crate::project_json
// Provides: {"impl_29"}
// Dependencies: {}
impl From < RunnableData > for Runnable { fn from (data : RunnableData) -> Self { Runnable { program : data . program , args : data . args , cwd : data . cwd , kind : data . kind . into () } } }
};
}
