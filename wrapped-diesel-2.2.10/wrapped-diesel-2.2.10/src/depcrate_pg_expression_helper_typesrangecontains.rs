// Generated macro for RangeContains (type)
macro_rules! Depcrate_pg_expression_helper_typesRangeContains {
() => {
// Module: crate::pg::expression::helper_types
// Provides: {"RangeContains"}
// Dependencies: {}
# [doc = " The return type of [`lhs.contains(rhs)`](super::expression_methods::PgRangeExpressionMethods::contains)"] # [doc = " for range expressions"] # [cfg (feature = "postgres_backend")] pub type RangeContains < Lhs , Rhs > = Grouped < super :: operators :: Contains < Lhs , AsExprOf < Rhs , < SqlTypeOf < Lhs > as super :: expression_methods :: RangeHelper > :: Inner > , > , > ;
};
}
