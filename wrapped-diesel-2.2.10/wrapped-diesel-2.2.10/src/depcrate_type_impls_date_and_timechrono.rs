// Generated macro for chrono (module)
macro_rules! Depcrate_type_impls_date_and_timechrono {
() => {
// Module: crate::type_impls::date_and_time
// Provides: {"chrono"}
// Dependencies: {}
# [cfg (feature = "chrono")] mod chrono { extern crate chrono ; use self :: chrono :: * ; use crate :: deserialize :: FromSqlRow ; use crate :: expression :: AsExpression ; use crate :: sql_types :: { Date , Interval , Time , Timestamp } ; # [derive (AsExpression , FromSqlRow)] # [diesel (foreign_derive)] # [diesel (sql_type = Date)] struct NaiveDateProxy (NaiveDate) ; # [derive (AsExpression , FromSqlRow)] # [diesel (foreign_derive)] # [diesel (sql_type = Time)] struct NaiveTimeProxy (NaiveTime) ; # [derive (AsExpression , FromSqlRow)] # [diesel (foreign_derive)] # [diesel (sql_type = Timestamp)] # [cfg_attr (feature = "postgres_backend" , diesel (sql_type = crate :: sql_types :: Timestamptz))] # [cfg_attr (feature = "mysql_backend" , diesel (sql_type = crate :: sql_types :: Datetime))] struct NaiveDateTimeProxy (NaiveDateTime) ; # [derive (FromSqlRow)] # [diesel (foreign_derive)] # [cfg_attr (any (feature = "postgres_backend" , feature = "sqlite") , derive (AsExpression))] # [cfg_attr (feature = "postgres_backend" , diesel (sql_type = crate :: sql_types :: Timestamptz))] # [cfg_attr (feature = "sqlite" , diesel (sql_type = crate :: sql_types :: TimestamptzSqlite))] struct DateTimeProxy < Tz : TimeZone > (DateTime < Tz >) ; # [derive (AsExpression , FromSqlRow)] # [diesel (foreign_derive)] # [diesel (sql_type = Interval)] struct DurationProxy (Duration) ; }
};
}
