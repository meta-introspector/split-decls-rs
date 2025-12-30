// Generated macro for list_contains_name (function)
macro_rules! Depcrate_attrlist_contains_name {
() => {
// Module: crate::attr
// Provides: {"list_contains_name"}
// Dependencies: {}
pub fn list_contains_name (items : & [MetaItemInner] , name : Symbol) -> bool { items . iter () . any (| item | item . has_name (name)) }
};
}
