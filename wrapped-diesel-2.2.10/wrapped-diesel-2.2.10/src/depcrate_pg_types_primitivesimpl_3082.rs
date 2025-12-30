// Generated macro for impl_3082 (impl)
macro_rules! Depcrate_pg_types_primitivesimpl_3082 {
() => {
// Module: crate::pg::types::primitives
// Provides: {"impl_3082"}
// Dependencies: {}
# [cfg (feature = "postgres_backend")] impl ToSql < sql_types :: Citext , Pg > for String { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Pg >) -> serialize :: Result { out . write_all (self . as_bytes ()) ? ; Ok (IsNull :: No) } }
};
}
