// Generated macro for FrameInfo (struct)
macro_rules! Depcrate_interpret_stackFrameInfo {
() => {
// Module: crate::interpret::stack
// Provides: {"FrameInfo"}
// Dependencies: {}
# [doc = " What we store about a frame in an interpreter backtrace."] # [derive (Clone , Debug)] pub struct FrameInfo < 'tcx > { pub instance : ty :: Instance < 'tcx > , pub span : Span , }
};
}
