// Generated macro for impl_1242 (impl)
macro_rules! Depcrate_query_builder_select_statement_dsl_implsimpl_1242 {
() => {
// Module: crate::query_builder::select_statement::dsl_impls
// Provides: {"impl_1242"}
// Dependencies: {}
impl < 'a , S , D , W , O , LOf , G , H , DB > BoxedDsl < 'a , DB > for SelectStatement < NoFromClause , S , D , W , O , LOf , G , H > where Self : AsQuery , DB : Backend , S : SelectClauseExpression < NoFromClause > + QueryFragment < DB > + Send + 'a , S :: Selection : ValidGrouping < G :: Expressions > , D : QueryFragment < DB > + Send + 'a , W : Into < BoxedWhereClause < 'a , DB > > , O : Into < Option < Box < dyn QueryFragment < DB > + Send + 'a > > > , LOf : IntoBoxedClause < 'a , DB , BoxedClause = BoxedLimitOffsetClause < 'a , DB > > , G : ValidGroupByClause + QueryFragment < DB > + Send + 'a , H : QueryFragment < DB > + Send + 'a , { type Output = BoxedSelectStatement < 'a , S :: SelectClauseSqlType , NoFromClause , DB , G :: Expressions > ; fn internal_into_boxed (self) -> Self :: Output { BoxedSelectStatement :: new_no_from_clause (self . select , self . from , Box :: new (self . distinct) , self . where_clause . into () , self . order . into () , self . limit_offset . into_boxed () , self . group_by , Box :: new (self . having) ,) } }
};
}
