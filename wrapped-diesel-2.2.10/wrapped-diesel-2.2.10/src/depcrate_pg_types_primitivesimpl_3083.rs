// Generated macro for impl_3083 (impl)
macro_rules! Depcrate_pg_types_primitivesimpl_3083 {
() => {
// Module: crate::pg::types::primitives
// Provides: {"impl_3083"}
// Dependencies: {}
# [cfg (feature = "postgres_backend")] impl ToSql < sql_types :: Citext , Pg > for str { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Pg >) -> serialize :: Result { out . write_all (self . as_bytes ()) ? ; Ok (IsNull :: No) } }
};
}
