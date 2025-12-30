// Generated macro for macro_2045 (macro)
macro_rules! Depcrate_sql_types_foldmacro_2045 {
() => {
// Module: crate::sql_types::fold
// Provides: {"macro_2045"}
// Dependencies: {}
foldable_impls ! { sql_types :: SmallInt => (sql_types :: BigInt , sql_types :: Numeric) , sql_types :: Integer => (sql_types :: BigInt , sql_types :: Numeric) , sql_types :: BigInt => (sql_types :: Numeric , sql_types :: Numeric) , sql_types :: Float => (sql_types :: Float , sql_types :: Double) , sql_types :: Double => (sql_types :: Double , sql_types :: Double) , sql_types :: Numeric => (sql_types :: Numeric , sql_types :: Numeric) , sql_types :: Interval => (sql_types :: Interval , sql_types :: Interval) , }
};
}
