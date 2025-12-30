// Generated macro for target_cpu (function)
macro_rules! Depcrate_gcc_utiltarget_cpu {
() => {
// Module: crate::gcc_util
// Provides: {"target_cpu"}
// Dependencies: {}
pub fn target_cpu (sess : & Session) -> & str { match sess . opts . cg . target_cpu { Some (ref name) => handle_native (name) , None => handle_native (sess . target . cpu . as_ref ()) , } }
};
}
