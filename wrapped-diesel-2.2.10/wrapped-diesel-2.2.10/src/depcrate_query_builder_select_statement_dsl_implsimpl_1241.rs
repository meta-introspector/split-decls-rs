// Generated macro for impl_1241 (impl)
macro_rules! Depcrate_query_builder_select_statement_dsl_implsimpl_1241 {
() => {
// Module: crate::query_builder::select_statement::dsl_impls
// Provides: {"impl_1241"}
// Dependencies: {}
impl < 'a , F , S , D , W , O , LOf , G , H , DB > BoxedDsl < 'a , DB > for SelectStatement < FromClause < F > , S , D , W , O , LOf , G , H > where Self : AsQuery , DB : Backend , F : QuerySource , S : SelectClauseExpression < FromClause < F > > + QueryFragment < DB > + Send + 'a , S :: Selection : ValidGrouping < G :: Expressions > , D : QueryFragment < DB > + Send + 'a , W : Into < BoxedWhereClause < 'a , DB > > , O : Into < Option < Box < dyn QueryFragment < DB > + Send + 'a > > > , LOf : IntoBoxedClause < 'a , DB , BoxedClause = BoxedLimitOffsetClause < 'a , DB > > , G : ValidGroupByClause + QueryFragment < DB > + Send + 'a , H : QueryFragment < DB > + Send + 'a , { type Output = BoxedSelectStatement < 'a , S :: SelectClauseSqlType , FromClause < F > , DB , G :: Expressions > ; fn internal_into_boxed (self) -> Self :: Output { BoxedSelectStatement :: new (self . select , self . from , Box :: new (self . distinct) , self . where_clause . into () , self . order . into () , self . limit_offset . into_boxed () , self . group_by , Box :: new (self . having) ,) } }
};
}
