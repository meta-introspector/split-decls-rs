// Generated macro for impl_288 (impl)
macro_rules! Depcrate_expression_array_comparisonimpl_288 {
() => {
// Module: crate::expression::array_comparison
// Provides: {"impl_288"}
// Dependencies: {}
impl < T , U > Expression for NotIn < T , U > where T : Expression , U : Expression < SqlType = T :: SqlType > , T :: SqlType : SqlType , sql_types :: is_nullable :: IsSqlTypeNullable < T :: SqlType > : sql_types :: MaybeNullableType < sql_types :: Bool > , { type SqlType = sql_types :: is_nullable :: MaybeNullable < sql_types :: is_nullable :: IsSqlTypeNullable < T :: SqlType > , sql_types :: Bool , > ; }
};
}
