// Generated macro for impl_3077 (impl)
macro_rules! Depcrate_pg_types_primitivesimpl_3077 {
() => {
// Module: crate::pg::types::primitives
// Provides: {"impl_3077"}
// Dependencies: {}
# [cfg (feature = "postgres_backend")] impl ToSql < sql_types :: CChar , Pg > for u8 { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Pg >) -> serialize :: Result { out . write_all (& [* self]) . map (| _ | IsNull :: No) . map_err (Into :: into) } }
};
}
