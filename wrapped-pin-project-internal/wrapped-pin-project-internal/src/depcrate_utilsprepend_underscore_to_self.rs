// Generated macro for prepend_underscore_to_self (function)
macro_rules! Depcrate_utilsprepend_underscore_to_self {
() => {
// Module: crate::utils
// Provides: {"prepend_underscore_to_self"}
// Dependencies: {}
pub (crate) fn prepend_underscore_to_self (ident : & mut Ident) -> bool { let modified = ident == "self" ; if modified { * ident = Ident :: new ("__self" , ident . span ()) ; } modified }
};
}
