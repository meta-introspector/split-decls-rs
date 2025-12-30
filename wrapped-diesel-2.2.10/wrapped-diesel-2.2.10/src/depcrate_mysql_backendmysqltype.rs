// Generated macro for MysqlType (enum)
macro_rules! Depcrate_mysql_backendMysqlType {
() => {
// Module: crate::mysql::backend
// Provides: {"MysqlType"}
// Dependencies: {}
# [allow (missing_debug_implementations)] # [doc = " Represents possible types, that can be transmitted as via the"] # [doc = " Mysql wire protocol"] # [derive (Debug , Hash , PartialEq , Eq , Clone , Copy)] # [non_exhaustive] pub enum MysqlType { # [doc = " A 8 bit signed integer"] Tiny , # [doc = " A 8 bit unsigned integer"] UnsignedTiny , # [doc = " A 16 bit signed integer"] Short , # [doc = " A 16 bit unsigned integer"] UnsignedShort , # [doc = " A 32 bit signed integer"] Long , # [doc = " A 32 bit unsigned integer"] UnsignedLong , # [doc = " A 64 bit signed integer"] LongLong , # [doc = " A 64 bit unsigned integer"] UnsignedLongLong , # [doc = " A 32 bit floating point number"] Float , # [doc = " A 64 bit floating point number"] Double , # [doc = " A fixed point decimal value"] Numeric , # [doc = " A datatype to store a time value"] Time , # [doc = " A datatype to store a date value"] Date , # [doc = " A datatype containing timestamp values ranging from"] # [doc = " '1000-01-01 00:00:00' to '9999-12-31 23:59:59'."] DateTime , # [doc = " A datatype containing timestamp values ranging from"] # [doc = " 1970-01-01 00:00:01' UTC to '2038-01-19 03:14:07' UTC."] Timestamp , # [doc = " A datatype for string values"] String , # [doc = " A datatype containing binary large objects"] Blob , # [doc = " A value containing a set of bit's"] Bit , # [doc = " A user defined set type"] Set , # [doc = " A user defined enum type"] Enum , }
};
}
