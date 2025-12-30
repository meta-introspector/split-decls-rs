// Generated macro for time (module)
macro_rules! Depcrate_type_impls_date_and_timetime {
() => {
// Module: crate::type_impls::date_and_time
// Provides: {"time"}
// Dependencies: {}
# [cfg (feature = "time")] mod time { use time :: { Date as NaiveDate , OffsetDateTime , PrimitiveDateTime , Time as NaiveTime } ; use crate :: deserialize :: FromSqlRow ; use crate :: expression :: AsExpression ; use crate :: sql_types :: { Date , Time , Timestamp } ; # [derive (AsExpression , FromSqlRow)] # [diesel (foreign_derive)] # [diesel (sql_type = Date)] struct NaiveDateProxy (NaiveDate) ; # [derive (AsExpression , FromSqlRow)] # [diesel (foreign_derive)] # [diesel (sql_type = Time)] struct NaiveTimeProxy (NaiveTime) ; # [derive (AsExpression , FromSqlRow)] # [diesel (foreign_derive)] # [diesel (sql_type = Timestamp)] # [cfg_attr (feature = "postgres_backend" , diesel (sql_type = crate :: sql_types :: Timestamptz))] # [cfg_attr (feature = "mysql_backend" , diesel (sql_type = crate :: sql_types :: Datetime))] struct NaiveDateTimeProxy (PrimitiveDateTime) ; # [derive (FromSqlRow)] # [diesel (foreign_derive)] # [cfg_attr (any (feature = "postgres_backend" , feature = "sqlite" , feature = "mysql_backend") , derive (AsExpression))] # [cfg_attr (feature = "postgres_backend" , diesel (sql_type = crate :: sql_types :: Timestamptz))] # [cfg_attr (feature = "sqlite" , diesel (sql_type = crate :: sql_types :: TimestamptzSqlite))] # [cfg_attr (feature = "mysql_backend" , diesel (sql_type = crate :: sql_types :: Datetime))] struct DateTimeProxy (OffsetDateTime) ; }
};
}
