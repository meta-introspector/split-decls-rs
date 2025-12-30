// Generated macro for impl_287 (impl)
macro_rules! Depcrate_expression_array_comparisonimpl_287 {
() => {
// Module: crate::expression::array_comparison
// Provides: {"impl_287"}
// Dependencies: {}
impl < T , U > Expression for In < T , U > where T : Expression , U : Expression < SqlType = T :: SqlType > , T :: SqlType : SqlType , sql_types :: is_nullable :: IsSqlTypeNullable < T :: SqlType > : sql_types :: MaybeNullableType < sql_types :: Bool > , { type SqlType = sql_types :: is_nullable :: MaybeNullable < sql_types :: is_nullable :: IsSqlTypeNullable < T :: SqlType > , sql_types :: Bool , > ; }
};
}
