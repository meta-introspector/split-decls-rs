// Generated macro for Timestamp (struct)
macro_rules! Depcrate_wrappersTimestamp {
() => {
// Module: crate::wrappers
// Provides: {"Timestamp"}
// Dependencies: {}
# [doc = " A wrapper type for [`jiff::Timestamp`]."] # [derive (Clone , Copy , Debug , Eq , Hash , PartialEq , PartialOrd , Ord , diesel :: deserialize :: FromSqlRow ,)] # [cfg_attr (any (feature = "mysql" , feature = "postgres" , feature = "sqlite") , derive (diesel :: expression :: AsExpression))] # [cfg_attr (feature = "mysql" , diesel (sql_type = diesel :: sql_types :: Datetime) ,)] # [cfg_attr (feature = "postgres" , diesel (sql_type = diesel :: sql_types :: Timestamptz) ,)] # [cfg_attr (feature = "sqlite" , diesel (sql_type = diesel :: sql_types :: TimestamptzSqlite) ,)] pub struct Timestamp (jiff :: Timestamp) ;
};
}
