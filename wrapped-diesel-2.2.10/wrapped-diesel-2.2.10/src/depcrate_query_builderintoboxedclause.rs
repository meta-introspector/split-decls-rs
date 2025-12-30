// Generated macro for IntoBoxedClause (trait)
macro_rules! Depcrate_query_builderIntoBoxedClause {
() => {
// Module: crate::query_builder
// Provides: {"IntoBoxedClause"}
// Dependencies: {}
# [doc = " A trait used to construct type erased boxed variant of the current query node"] # [doc = ""] # [doc = " Mainly useful for implementing third party backends"] pub trait IntoBoxedClause < 'a , DB > { # [doc = " Resulting type"] type BoxedClause ; # [doc = " Convert the given query node in it's boxed representation"] fn into_boxed (self) -> Self :: BoxedClause ; }
};
}
