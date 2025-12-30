// Generated macro for macro_2046 (macro)
macro_rules! Depcrate_sql_types_foldmacro_2046 {
() => {
// Module: crate::sql_types::fold
// Provides: {"macro_2046"}
// Dependencies: {}
# [cfg (feature = "mysql_backend")] foldable_impls ! { sql_types :: Unsigned < sql_types :: SmallInt > => (sql_types :: Unsigned < sql_types :: BigInt >, sql_types :: Numeric) , sql_types :: Unsigned < sql_types :: Integer > => (sql_types :: Unsigned < sql_types :: BigInt >, sql_types :: Numeric) , sql_types :: Unsigned < sql_types :: BigInt > => (sql_types :: Numeric , sql_types :: Numeric) , }
};
}
