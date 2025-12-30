// Generated macro for impl_3144 (impl)
macro_rules! Depcrate_pg_types_uuidimpl_3144 {
() => {
// Module: crate::pg::types::uuid
// Provides: {"impl_3144"}
// Dependencies: {}
# [cfg (all (feature = "postgres_backend" , feature = "uuid"))] impl ToSql < Uuid , Pg > for uuid :: Uuid { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Pg >) -> serialize :: Result { out . write_all (self . as_bytes ()) . map (| _ | IsNull :: No) . map_err (Into :: into) } }
};
}
