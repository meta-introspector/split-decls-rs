// Generated macro for impl_1860 (impl)
macro_rules! Depcrate_query_source_joinsimpl_1860 {
() => {
// Module: crate::query_source::joins
// Provides: {"impl_1860"}
// Dependencies: {}
impl < Left , Right , Kind > std :: fmt :: Debug for Join < Left , Right , Kind > where Left : QuerySource , FromClause < Left > : std :: fmt :: Debug , Right : QuerySource , FromClause < Right > : std :: fmt :: Debug , Kind : std :: fmt :: Debug , { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . debug_struct ("Join") . field ("left" , & self . left) . field ("right" , & self . right) . field ("kind" , & self . kind) . finish () } }
};
}
