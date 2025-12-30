// Generated macro for impl_841 (impl)
macro_rules! Depcrate_query_builder_debug_queryimpl_841 {
() => {
// Module: crate::query_builder::debug_query
// Provides: {"impl_841"}
// Dependencies: {}
impl < T , DB > Debug for DebugQuery < '_ , T , DB > where DB : Backend + Default , DB :: QueryBuilder : Default , T : QueryFragment < DB > , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { debug (self . query , f) } }
};
}
