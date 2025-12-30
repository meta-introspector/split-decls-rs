// Generated macro for impl_1887 (impl)
macro_rules! Depcrate_query_source_joinsimpl_1887 {
() => {
// Module: crate::query_source::joins
// Provides: {"impl_1887"}
// Dependencies: {}
impl < Rhs , Kind , On1 , On2 , Lhs > InternalJoinDsl < Rhs , Kind , On1 > for OnClauseWrapper < Lhs , On2 > where Lhs : InternalJoinDsl < Rhs , Kind , On1 > , { type Output = OnClauseWrapper < < Lhs as InternalJoinDsl < Rhs , Kind , On1 > > :: Output , On2 > ; fn join (self , rhs : Rhs , kind : Kind , on : On1) -> Self :: Output { OnClauseWrapper { source : self . source . join (rhs , kind , on) , on : self . on , } } }
};
}
