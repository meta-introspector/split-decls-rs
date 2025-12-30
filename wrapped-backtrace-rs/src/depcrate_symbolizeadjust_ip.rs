// Generated macro for adjust_ip (function)
macro_rules! Depcrate_symbolizeadjust_ip {
() => {
// Module: crate::symbolize
// Provides: {"adjust_ip"}
// Dependencies: {}
fn adjust_ip (a : * mut c_void) -> * mut c_void { if a . is_null () { a } else { (a as usize - 1) as * mut c_void } }
};
}
