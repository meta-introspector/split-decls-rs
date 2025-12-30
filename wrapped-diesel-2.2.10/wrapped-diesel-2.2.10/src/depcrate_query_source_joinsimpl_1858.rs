// Generated macro for impl_1858 (impl)
macro_rules! Depcrate_query_source_joinsimpl_1858 {
() => {
// Module: crate::query_source::joins
// Provides: {"impl_1858"}
// Dependencies: {}
impl < Left , Right , Kind > Clone for Join < Left , Right , Kind > where Left : QuerySource , FromClause < Left > : Clone , Right : QuerySource , FromClause < Right > : Clone , Kind : Clone , { fn clone (& self) -> Self { Self { left : self . left . clone () , right : self . right . clone () , kind : self . kind . clone () , } } }
};
}
