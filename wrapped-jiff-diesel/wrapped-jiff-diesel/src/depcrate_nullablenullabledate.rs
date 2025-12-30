// Generated macro for NullableDate (struct)
macro_rules! Depcrate_nullableNullableDate {
() => {
// Module: crate::nullable
// Provides: {"NullableDate"}
// Dependencies: {}
# [doc = " A wrapper type for `Option<jiff::civil::Date>`."] # [doc = ""] # [doc = " This can be used when deriving [`diesel::deserialize::Queryable`]"] # [doc = " or [`diesel::deserialize::QueryableByName`] trait implementations."] # [derive (Clone , Copy , Debug , diesel :: expression :: AsExpression , diesel :: deserialize :: FromSqlRow ,)] # [diesel (sql_type = diesel :: sql_types :: Nullable < diesel :: sql_types :: Date >)] pub struct NullableDate (Option < crate :: Date >) ;
};
}
