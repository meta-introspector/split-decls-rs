// Generated macro for impl_1832 (impl)
macro_rules! Depcrate_query_source_aliasing_joinsimpl_1832 {
() => {
// Module: crate::query_source::aliasing::joins
// Provides: {"impl_1832"}
// Dependencies: {}
impl < S , Rhs , Kind , On > InternalJoinDsl < Rhs , Kind , On > for Alias < S > where Self : AsQuery , < Self as AsQuery > :: Query : InternalJoinDsl < Rhs , Kind , On > , { type Output = < < Self as AsQuery > :: Query as InternalJoinDsl < Rhs , Kind , On > > :: Output ; fn join (self , rhs : Rhs , kind : Kind , on : On) -> Self :: Output { self . as_query () . join (rhs , kind , on) } }
};
}
