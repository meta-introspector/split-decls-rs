// Generated macro for NumericRepresentation (enum)
macro_rules! Depcrate_mysql_valueNumericRepresentation {
() => {
// Module: crate::mysql::value
// Provides: {"NumericRepresentation"}
// Dependencies: {}
# [doc = " Represents all possible forms MySQL transmits integers"] # [derive (Debug , Clone , Copy)] # [non_exhaustive] pub enum NumericRepresentation < 'a > { # [doc = " Corresponds to `MYSQL_TYPE_TINY`"] Tiny (i8) , # [doc = " Corresponds to `MYSQL_TYPE_SHORT`"] Small (i16) , # [doc = " Corresponds to `MYSQL_TYPE_INT24` and `MYSQL_TYPE_LONG`"] Medium (i32) , # [doc = " Corresponds to `MYSQL_TYPE_LONGLONG`"] Big (i64) , # [doc = " Corresponds to `MYSQL_TYPE_FLOAT`"] Float (f32) , # [doc = " Corresponds to `MYSQL_TYPE_DOUBLE`"] Double (f64) , # [doc = " Corresponds to `MYSQL_TYPE_DECIMAL` and `MYSQL_TYPE_NEWDECIMAL`"] Decimal (& 'a [u8]) , }
};
}
