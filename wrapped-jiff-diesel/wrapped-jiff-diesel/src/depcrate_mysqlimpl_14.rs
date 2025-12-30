// Generated macro for impl_14 (impl)
macro_rules! Depcrate_mysqlimpl_14 {
() => {
// Module: crate::mysql
// Provides: {"impl_14"}
// Dependencies: {}
impl ToSql < sql_types :: Time , Mysql > for Time { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Mysql > ,) -> serialize :: Result { let mut time = self . to_jiff () ; if time . nanosecond () != 0 { time = time . round (jiff :: Unit :: Microsecond) ? ; } let mysql_time = MysqlTime :: new (0 , 0 , 0 , time . hour () . unsigned_abs () . into () , time . minute () . unsigned_abs () . into () , time . second () . unsigned_abs () . into () , (time . subsec_nanosecond () . unsigned_abs () / 1_000) . into () , false , MysqlTimestampType :: MYSQL_TIMESTAMP_TIME , 0 ,) ; < MysqlTime as ToSql < sql_types :: Time , Mysql > > :: to_sql (& mysql_time , & mut out . reborrow () ,) } }
};
}
