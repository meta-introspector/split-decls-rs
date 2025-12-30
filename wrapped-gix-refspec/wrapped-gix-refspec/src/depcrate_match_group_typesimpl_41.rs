// Generated macro for impl_41 (impl)
macro_rules! Depcrate_match_group_typesimpl_41 {
() => {
// Module: crate::match_group::types
// Provides: {"impl_41"}
// Dependencies: {}
impl SourceRef < '_ > { # [doc = " Create a fully owned instance by consuming this one."] pub fn into_owned (self) -> Source { match self { SourceRef :: ObjectId (id) => Source :: ObjectId (id) , SourceRef :: FullName (name) => Source :: FullName (name . into_owned () . into ()) , } } }
};
}
