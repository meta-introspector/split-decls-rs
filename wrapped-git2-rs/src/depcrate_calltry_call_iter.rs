// Generated macro for try_call_iter (macro)
macro_rules! Depcrate_calltry_call_iter {
() => {
// Module: crate::call
// Provides: {"try_call_iter"}
// Dependencies: {}
macro_rules ! try_call_iter { ($ ($ f : tt) *) => { match call ! ($ ($ f) *) { 0 => { } raw :: GIT_ITEROVER => return None , e => return Some (Err (crate :: call :: last_error (e))) } } }
};
}
