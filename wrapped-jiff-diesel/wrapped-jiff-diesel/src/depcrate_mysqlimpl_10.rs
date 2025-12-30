// Generated macro for impl_10 (impl)
macro_rules! Depcrate_mysqlimpl_10 {
() => {
// Module: crate::mysql
// Provides: {"impl_10"}
// Dependencies: {}
impl ToSql < sql_types :: Timestamp , Mysql > for DateTime { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Mysql > ,) -> serialize :: Result { let mut dt = self . to_jiff () ; if dt . nanosecond () != 0 { dt = dt . round (jiff :: Unit :: Microsecond) ? ; } let mysql_time = MysqlTime :: new (dt . year () . try_into () ? , dt . month () . unsigned_abs () . into () , dt . day () . unsigned_abs () . into () , dt . hour () . unsigned_abs () . into () , dt . minute () . unsigned_abs () . into () , dt . second () . unsigned_abs () . into () , (dt . subsec_nanosecond () . unsigned_abs () / 1_000) . into () , false , MysqlTimestampType :: MYSQL_TIMESTAMP_DATETIME , 0 ,) ; < MysqlTime as ToSql < sql_types :: Timestamp , Mysql > > :: to_sql (& mysql_time , & mut out . reborrow () ,) } }
};
}
