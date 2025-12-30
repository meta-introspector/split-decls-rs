// Generated macro for impl_12 (impl)
macro_rules! Depcrate_mysqlimpl_12 {
() => {
// Module: crate::mysql
// Provides: {"impl_12"}
// Dependencies: {}
impl ToSql < sql_types :: Date , Mysql > for Date { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Mysql > ,) -> serialize :: Result { let date = self . to_jiff () ; let mysql_time = MysqlTime :: new (date . year () . try_into () ? , date . month () . unsigned_abs () . into () , date . day () . unsigned_abs () . into () , 0 , 0 , 0 , 0 , false , MysqlTimestampType :: MYSQL_TIMESTAMP_DATE , 0 ,) ; < MysqlTime as ToSql < sql_types :: Date , Mysql > > :: to_sql (& mysql_time , & mut out . reborrow () ,) } }
};
}
