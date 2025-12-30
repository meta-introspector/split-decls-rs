// Generated macro for MysqlValue (struct)
macro_rules! Depcrate_mysql_valueMysqlValue {
() => {
// Module: crate::mysql::value
// Provides: {"MysqlValue"}
// Dependencies: {}
# [doc = " Raw mysql value as received from the database"] # [derive (Clone , Debug)] pub struct MysqlValue < 'a > { raw : & 'a [u8] , tpe : MysqlType , }
};
}
