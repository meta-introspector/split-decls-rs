// Generated macro for impl_1158 (impl)
macro_rules! Depcrate_query_builder_select_statement_boxedimpl_1158 {
() => {
// Module: crate::query_builder::select_statement::boxed
// Provides: {"impl_1158"}
// Dependencies: {}
impl < 'a , ST , QS : QuerySource , DB , GB > BoxedSelectStatement < 'a , ST , FromClause < QS > , DB , GB > { # [allow (clippy :: too_many_arguments)] pub (crate) fn new < S , G > (select : S , from : FromClause < QS > , distinct : Box < dyn QueryFragment < DB > + Send + 'a > , where_clause : BoxedWhereClause < 'a , DB > , order : Option < Box < dyn QueryFragment < DB > + Send + 'a > > , limit_offset : BoxedLimitOffsetClause < 'a , DB > , group_by : G , having : Box < dyn QueryFragment < DB > + Send + 'a > ,) -> Self where DB : Backend , G : ValidGroupByClause < Expressions = GB > + QueryFragment < DB > + Send + 'a , S : SelectClauseExpression < FromClause < QS > , SelectClauseSqlType = ST > + QueryFragment < DB > + Send + 'a , S :: Selection : ValidGrouping < GB > , { BoxedSelectStatement { select : Box :: new (select) , from , distinct , where_clause , order , limit_offset , group_by : Box :: new (group_by) , having , _marker : PhantomData , } } }
};
}
