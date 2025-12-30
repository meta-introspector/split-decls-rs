// Generated macro for LoadRet (type)
macro_rules! Depcrate_query_dsl_load_dslLoadRet {
() => {
// Module: crate::query_dsl::load_dsl
// Provides: {"LoadRet"}
// Dependencies: {}
# [doc (hidden)] # [cfg (all (feature = "with-deprecated" , not (feature = "without-deprecated")))] # [deprecated (note = "Use `LoadQuery::Iter` directly")] pub type LoadRet < 'conn , 'query , Q , C , U , B = DefaultLoadingMode > = < Q as LoadQuery < 'query , C , U , B > > :: RowIter < 'conn > ;
};
}
