// Generated macro for impl_3075 (impl)
macro_rules! Depcrate_pg_types_primitivesimpl_3075 {
() => {
// Module: crate::pg::types::primitives
// Provides: {"impl_3075"}
// Dependencies: {}
# [cfg (feature = "postgres_backend")] impl ToSql < sql_types :: Bool , Pg > for bool { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Pg >) -> serialize :: Result { out . write_all (& [* self as u8]) . map (| _ | IsNull :: No) . map_err (Into :: into) } }
};
}
