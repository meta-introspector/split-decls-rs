// Generated macro for InPlaceCollect (trait)
macro_rules! Depcrate_vec_in_place_collectInPlaceCollect {
() => {
// Module: crate::vec::in_place_collect
// Provides: {"InPlaceCollect"}
// Dependencies: {}
# [doc = " This provides a shorthand for the source type since local type aliases aren't a thing."] # [rustc_specialization_trait] trait InPlaceCollect : SourceIter < Source : AsVecIntoIter > + InPlaceIterable { type Src ; }
};
}
