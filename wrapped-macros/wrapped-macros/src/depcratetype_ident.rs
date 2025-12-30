// Generated macro for type_ident (function)
macro_rules! Depcratetype_ident {
() => {
// Module: crate
// Provides: {"type_ident"}
// Dependencies: {}
fn type_ident (ty : impl AsRef < syn :: Type >) -> String { let mut ident = String :: new () ; let ty = ty . as_ref () ; let ty = format ! ("{}" , quote ! (# ty)) ; ty . split_whitespace () . for_each (| t | ident . push_str (t)) ; ident }
};
}
