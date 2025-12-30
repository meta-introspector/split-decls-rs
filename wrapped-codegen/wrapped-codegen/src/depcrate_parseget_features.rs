// Generated macro for get_features (function)
macro_rules! Depcrate_parseget_features {
() => {
// Module: crate::parse
// Provides: {"get_features"}
// Dependencies: {}
fn get_features (attrs : & [Attribute] , base : & [Attribute]) -> Vec < Attribute > { let mut ret = clone_features (base) ; for attr in attrs { if attr . path () . is_ident ("cfg") { ret . push (parse_quote ! (# attr)) ; } } ret }
};
}
