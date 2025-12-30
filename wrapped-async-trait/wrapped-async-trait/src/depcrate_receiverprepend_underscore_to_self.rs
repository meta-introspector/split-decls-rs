// Generated macro for prepend_underscore_to_self (function)
macro_rules! Depcrate_receiverprepend_underscore_to_self {
() => {
// Module: crate::receiver
// Provides: {"prepend_underscore_to_self"}
// Dependencies: {}
fn prepend_underscore_to_self (ident : & mut Ident) -> bool { let modified = ident == "self" ; if modified { * ident = Ident :: new ("__self" , ident . span ()) ; } modified }
};
}
