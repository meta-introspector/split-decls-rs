// Generated macro for inside_proc_macro (function)
macro_rules! Depcrate_detectioninside_proc_macro {
() => {
// Module: crate::detection
// Provides: {"inside_proc_macro"}
// Dependencies: {}
pub (crate) fn inside_proc_macro () -> bool { match WORKS . load (Ordering :: Relaxed) { 1 => return false , 2 => return true , _ => { } } INIT . call_once (initialize) ; inside_proc_macro () }
};
}
