// Generated macro for NullableDateTime (struct)
macro_rules! Depcrate_nullableNullableDateTime {
() => {
// Module: crate::nullable
// Provides: {"NullableDateTime"}
// Dependencies: {}
# [doc = " A wrapper type for `Option<jiff::civil::DateTime>`."] # [doc = ""] # [doc = " This can be used when deriving [`diesel::deserialize::Queryable`]"] # [doc = " or [`diesel::deserialize::QueryableByName`] trait implementations."] # [derive (Clone , Copy , Debug , diesel :: expression :: AsExpression , diesel :: deserialize :: FromSqlRow ,)] # [diesel (sql_type = diesel :: sql_types :: Nullable < diesel :: sql_types :: Timestamp >)] pub struct NullableDateTime (Option < crate :: DateTime >) ;
};
}
