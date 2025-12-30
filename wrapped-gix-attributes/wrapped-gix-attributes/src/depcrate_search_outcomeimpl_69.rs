// Generated macro for impl_69 (impl)
macro_rules! Depcrate_search_outcomeimpl_69 {
() => {
// Module: crate::search::outcome
// Provides: {"impl_69"}
// Dependencies: {}
impl MatchKind { # [doc = " return the id of the macro that resolved us, or `None` if that didn't happen."] pub fn source_id (& self) -> Option < AttributeId > { match self { MatchKind :: Attribute { macro_id : id } | MatchKind :: Macro { parent_macro_id : id } => * id , } } }
};
}
