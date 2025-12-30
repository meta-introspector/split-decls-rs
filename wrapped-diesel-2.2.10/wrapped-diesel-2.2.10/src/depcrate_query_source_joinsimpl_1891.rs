// Generated macro for impl_1891 (impl)
macro_rules! Depcrate_query_source_joinsimpl_1891 {
() => {
// Module: crate::query_source::joins
// Provides: {"impl_1891"}
// Dependencies: {}
impl < Join , On > ToInnerJoin for JoinOn < Join , On > where Join : ToInnerJoin , { type InnerJoin = JoinOn < Join :: InnerJoin , On > ; }
};
}
