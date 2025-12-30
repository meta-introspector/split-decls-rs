// Generated macro for ClippyCheckOutput (enum)
macro_rules! Depcrate_outputClippyCheckOutput {
() => {
// Module: crate::output
// Provides: {"ClippyCheckOutput"}
// Dependencies: {}
# [doc = " A single emitted output from clippy being executed on a crate. It may either be a"] # [doc = " `ClippyWarning`, or a `RustcIce` caused by a panic within clippy. A crate may have many"] # [doc = " `ClippyWarning`s but a maximum of one `RustcIce` (at which point clippy halts execution)."] # [derive (Debug)] pub enum ClippyCheckOutput { ClippyWarning (ClippyWarning) , RustcIce (RustcIce) , }
};
}
