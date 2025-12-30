// Generated macro for retained_ptr_cast (function)
macro_rules! Depcrate_utilretained_ptr_cast {
() => {
// Module: crate::util
// Provides: {"retained_ptr_cast"}
// Dependencies: {}
pub (crate) fn retained_ptr_cast < T : ? Sized > (objects : * mut Retained < T >) -> * mut NonNull < T > { objects . cast () }
};
}
