// Generated macro for impl_1863 (impl)
macro_rules! Depcrate_query_source_joinsimpl_1863 {
() => {
// Module: crate::query_source::joins
// Provides: {"impl_1863"}
// Dependencies: {}
impl < Left , Right , Kind > Join < Left , Right , Kind > where Left : QuerySource , Right : QuerySource , { pub (crate) fn new (left : Left , right : Right , kind : Kind) -> Self { Join { left : FromClause :: new (left) , right : FromClause :: new (right) , kind , } } pub (crate) fn on < On > (self , on : On) -> JoinOn < Self , On > { JoinOn { join : self , on : on } } }
};
}
