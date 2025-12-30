// Generated macro for impl_2522 (impl)
macro_rules! Depcrate_mysql_types_primitivesimpl_2522 {
() => {
// Module: crate::mysql::types::primitives
// Provides: {"impl_2522"}
// Dependencies: {}
# [cfg (feature = "mysql_backend")] impl Queryable < Text , Mysql > for * const str { type Row = Self ; fn build (row : Self :: Row) -> deserialize :: Result < Self > { Ok (row) } }
};
}
