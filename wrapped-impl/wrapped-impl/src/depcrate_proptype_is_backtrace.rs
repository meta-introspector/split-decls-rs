// Generated macro for type_is_backtrace (function)
macro_rules! Depcrate_proptype_is_backtrace {
() => {
// Module: crate::prop
// Provides: {"type_is_backtrace"}
// Dependencies: {}
fn type_is_backtrace (ty : & Type) -> bool { let path = match ty { Type :: Path (ty) => & ty . path , _ => return false , } ; let last = path . segments . last () . unwrap () ; last . ident == "Backtrace" && last . arguments . is_empty () }
};
}
