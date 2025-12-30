// Generated macro for NullableTime (struct)
macro_rules! Depcrate_nullableNullableTime {
() => {
// Module: crate::nullable
// Provides: {"NullableTime"}
// Dependencies: {}
# [doc = " A wrapper type for `Option<jiff::civil::Time>`."] # [doc = ""] # [doc = " This can be used when deriving [`diesel::deserialize::Queryable`]"] # [doc = " or [`diesel::deserialize::QueryableByName`] trait implementations."] # [derive (Clone , Copy , Debug , diesel :: expression :: AsExpression , diesel :: deserialize :: FromSqlRow ,)] # [diesel (sql_type = diesel :: sql_types :: Nullable < diesel :: sql_types :: Time >)] pub struct NullableTime (Option < crate :: Time >) ;
};
}
