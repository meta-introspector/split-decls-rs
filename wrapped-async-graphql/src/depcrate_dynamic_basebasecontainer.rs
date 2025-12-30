// Generated macro for BaseContainer (trait)
macro_rules! Depcrate_dynamic_baseBaseContainer {
() => {
// Module: crate::dynamic::base
// Provides: {"BaseContainer"}
// Dependencies: {}
pub (crate) trait BaseContainer { type FieldType : BaseField ; fn name (& self) -> & str ; fn graphql_type (& self) -> & str ; fn field (& self , name : & str) -> Option < & Self :: FieldType > ; }
};
}
