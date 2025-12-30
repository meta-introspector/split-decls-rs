// Generated macro for data_types (module)
macro_rules! Depcrate_mysqldata_types {
() => {
// Module: crate::mysql
// Provides: {"data_types"}
// Dependencies: {}
# [doc = " Data structures for MySQL types which have no corresponding Rust type"] # [doc = ""] # [doc = " Most of these types are used to implement `ToSql` and `FromSql` for higher"] # [doc = " level types."] pub mod data_types { # [doc (inline)] pub use super :: types :: date_and_time :: { MysqlTime , MysqlTimestampType } ; }
};
}
