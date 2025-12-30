// Generated macro for impl_4009 (impl)
macro_rules! Depcrate_sqlite_types_numericimpl_4009 {
() => {
// Module: crate::sqlite::types::numeric
// Provides: {"impl_4009"}
// Dependencies: {}
# [cfg (all (feature = "sqlite" , feature = "numeric"))] impl FromSql < Numeric , Sqlite > for BigDecimal { fn from_sql (bytes : SqliteValue < '_ , '_ , '_ >) -> deserialize :: Result < Self > { let x = < f64 as FromSql < Double , Sqlite > > :: from_sql (bytes) ? ; BigDecimal :: from_f64 (x) . ok_or_else (| | format ! ("{x} is not valid decimal number ") . into ()) } }
};
}
