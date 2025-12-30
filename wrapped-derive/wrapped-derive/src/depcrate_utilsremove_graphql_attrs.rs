// Generated macro for remove_graphql_attrs (function)
macro_rules! Depcrate_utilsremove_graphql_attrs {
() => {
// Module: crate::utils
// Provides: {"remove_graphql_attrs"}
// Dependencies: {}
pub fn remove_graphql_attrs (attrs : & mut Vec < Attribute >) { if let Some ((idx , _)) = attrs . iter () . enumerate () . find (| (_ , a) | a . path () . is_ident ("graphql")) { attrs . remove (idx) ; } }
};
}
