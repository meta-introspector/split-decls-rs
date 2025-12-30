// Generated macro for impl_8 (impl)
macro_rules! Depcrate_mysqlimpl_8 {
() => {
// Module: crate::mysql
// Provides: {"impl_8"}
// Dependencies: {}
impl ToSql < sql_types :: Datetime , Mysql > for Timestamp { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Mysql > ,) -> serialize :: Result { let mut dt = UTC . to_datetime (self . to_jiff ()) ; if dt . nanosecond () != 0 { dt = dt . round (jiff :: Unit :: Microsecond) ? ; } let mysql_time = MysqlTime :: new (dt . year () . try_into () ? , dt . month () . unsigned_abs () . into () , dt . day () . unsigned_abs () . into () , dt . hour () . unsigned_abs () . into () , dt . minute () . unsigned_abs () . into () , dt . second () . unsigned_abs () . into () , (dt . subsec_nanosecond () . unsigned_abs () / 1_000) . into () , false , MysqlTimestampType :: MYSQL_TIMESTAMP_DATETIME_TZ , 0 ,) ; < MysqlTime as ToSql < sql_types :: Datetime , Mysql > > :: to_sql (& mysql_time , & mut out . reborrow () ,) } }
};
}
