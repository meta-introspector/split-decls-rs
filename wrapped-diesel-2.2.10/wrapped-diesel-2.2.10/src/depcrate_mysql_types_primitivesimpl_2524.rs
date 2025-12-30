// Generated macro for impl_2524 (impl)
macro_rules! Depcrate_mysql_types_primitivesimpl_2524 {
() => {
// Module: crate::mysql::types::primitives
// Provides: {"impl_2524"}
// Dependencies: {}
# [cfg (feature = "mysql_backend")] impl Queryable < Binary , Mysql > for * const [u8] { type Row = Self ; fn build (row : Self :: Row) -> deserialize :: Result < Self > { Ok (row) } }
};
}
