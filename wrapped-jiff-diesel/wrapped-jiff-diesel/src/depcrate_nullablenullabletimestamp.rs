// Generated macro for NullableTimestamp (struct)
macro_rules! Depcrate_nullableNullableTimestamp {
() => {
// Module: crate::nullable
// Provides: {"NullableTimestamp"}
// Dependencies: {}
# [doc = " A wrapper type for `Option<jiff::Timestamp>`."] # [doc = ""] # [doc = " This can be used when deriving [`diesel::deserialize::Queryable`]"] # [doc = " or [`diesel::deserialize::QueryableByName`] trait implementations."] # [derive (Clone , Copy , Debug , diesel :: deserialize :: FromSqlRow)] # [cfg_attr (any (feature = "mysql" , feature = "postgres" , feature = "sqlite") , derive (diesel :: expression :: AsExpression))] # [cfg_attr (feature = "mysql" , diesel (sql_type = diesel :: sql_types :: Nullable < diesel :: sql_types :: Datetime >) ,)] # [cfg_attr (feature = "postgres" , diesel (sql_type = diesel :: sql_types :: Nullable < diesel :: sql_types :: Timestamptz >) ,)] # [cfg_attr (feature = "sqlite" , diesel (sql_type = diesel :: sql_types :: Nullable < diesel :: sql_types :: TimestamptzSqlite >) ,)] pub struct NullableTimestamp (Option < crate :: Timestamp >) ;
};
}
