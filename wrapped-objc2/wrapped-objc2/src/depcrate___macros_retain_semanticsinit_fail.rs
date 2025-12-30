// Generated macro for init_fail (function)
macro_rules! Depcrate___macros_retain_semanticsinit_fail {
() => {
// Module: crate::__macros::retain_semantics
// Provides: {"init_fail"}
// Dependencies: {}
# [cold] # [track_caller] fn init_fail (receiver : * mut AnyObject , sel : Sel) -> ! { if receiver . is_null () { panic ! ("failed allocating object") } else { if sel == sel ! (init) { panic ! ("failed initializing object") } else { panic ! ("failed initializing object with -{sel}") } } }
};
}
