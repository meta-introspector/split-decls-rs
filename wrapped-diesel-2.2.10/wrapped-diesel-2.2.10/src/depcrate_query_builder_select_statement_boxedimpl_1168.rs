// Generated macro for impl_1168 (impl)
macro_rules! Depcrate_query_builder_select_statement_boxedimpl_1168 {
() => {
// Module: crate::query_builder::select_statement::boxed
// Provides: {"impl_1168"}
// Dependencies: {}
impl < 'a , ST , QS , DB , Rhs , Kind , On , GB > InternalJoinDsl < Rhs , Kind , On > for BoxedSelectStatement < 'a , ST , FromClause < QS > , DB , GB > where QS : QuerySource , Rhs : QuerySource , JoinOn < Join < QS , Rhs , Kind > , On > : QuerySource , BoxedSelectStatement < 'a , ST , FromClause < JoinOn < Join < QS , Rhs , Kind > , On > > , DB , GB > : AsQuery , { type Output = BoxedSelectStatement < 'a , ST , FromClause < JoinOn < Join < QS , Rhs , Kind > , On > > , DB , GB > ; fn join (self , rhs : Rhs , kind : Kind , on : On) -> Self :: Output { BoxedSelectStatement { select : self . select , from : FromClause :: new (Join :: new (self . from . source , rhs , kind) . on (on)) , distinct : self . distinct , where_clause : self . where_clause , order : self . order , limit_offset : self . limit_offset , group_by : self . group_by , having : self . having , _marker : PhantomData , } } }
};
}
