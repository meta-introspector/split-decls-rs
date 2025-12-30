// Generated macro for has_attr (function)
macro_rules! Depcratehas_attr {
() => {
// Module: crate
// Provides: {"has_attr"}
// Dependencies: {}
pub fn has_attr (attrs : & [hir :: Attribute] , symbol : Symbol) -> bool { attrs . iter () . any (| attr | attr . has_name (symbol)) }
};
}
