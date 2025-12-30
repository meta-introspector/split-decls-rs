// Generated macro for impl_42 (impl)
macro_rules! Depcrate_match_group_typesimpl_42 {
() => {
// Module: crate::match_group::types
// Provides: {"impl_42"}
// Dependencies: {}
impl std :: fmt :: Display for SourceRef < '_ > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match self { SourceRef :: FullName (name) => name . fmt (f) , SourceRef :: ObjectId (id) => id . fmt (f) , } } }
};
}
