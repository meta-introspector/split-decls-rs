// Generated macro for impl_1534 (impl)
macro_rules! Depcrate_query_builderimpl_1534 {
() => {
// Module: crate::query_builder
// Provides: {"impl_1534"}
// Dependencies: {}
impl < T : Query > AsQuery for T { type SqlType = < T as Query > :: SqlType ; type Query = T ; fn as_query (self) -> < T as AsQuery > :: Query { self } }
};
}
