// Generated macro for retained_ptr_cast_const (function)
macro_rules! Depcrate_utilretained_ptr_cast_const {
() => {
// Module: crate::util
// Provides: {"retained_ptr_cast_const"}
// Dependencies: {}
pub (crate) fn retained_ptr_cast_const < T : ? Sized > (objects : * const Retained < T >) -> * mut NonNull < T > { retained_ptr_cast (objects as * mut Retained < T >) }
};
}
