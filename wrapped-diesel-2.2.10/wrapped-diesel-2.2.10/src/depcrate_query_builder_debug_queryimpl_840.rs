// Generated macro for impl_840 (impl)
macro_rules! Depcrate_query_builder_debug_queryimpl_840 {
() => {
// Module: crate::query_builder::debug_query
// Provides: {"impl_840"}
// Dependencies: {}
impl < T , DB > Display for DebugQuery < '_ , T , DB > where DB : Backend + Default , DB :: QueryBuilder : Default , T : QueryFragment < DB > , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { display (self . query , f) } }
};
}
