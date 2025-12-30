// Generated macro for only_field_ty (function)
macro_rules! Depcrateonly_field_ty {
() => {
// Module: crate
// Provides: {"only_field_ty"}
// Dependencies: {}
fn only_field_ty (fields : & Fields) -> Result < & Type > { let is_trivial = decide_trivial (fields) ? ; let mut only_field = None ; for field in fields { if ! is_trivial (field) ? { if only_field . take () . is_some () { break ; } only_field = Some (& field . ty) ; } } only_field . ok_or_else (| | { Error :: new (Span :: call_site () , "RefCast requires a struct with a single field" ,) }) }
};
}
