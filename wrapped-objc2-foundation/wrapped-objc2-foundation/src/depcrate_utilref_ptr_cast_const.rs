// Generated macro for ref_ptr_cast_const (function)
macro_rules! Depcrate_utilref_ptr_cast_const {
() => {
// Module: crate::util
// Provides: {"ref_ptr_cast_const"}
// Dependencies: {}
pub (crate) fn ref_ptr_cast_const < T : ? Sized > (objects : * const & T) -> * mut NonNull < T > { (objects as * mut & T) . cast () }
};
}
