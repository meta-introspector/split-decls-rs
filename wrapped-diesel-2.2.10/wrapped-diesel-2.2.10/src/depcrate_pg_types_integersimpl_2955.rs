// Generated macro for impl_2955 (impl)
macro_rules! Depcrate_pg_types_integersimpl_2955 {
() => {
// Module: crate::pg::types::integers
// Provides: {"impl_2955"}
// Dependencies: {}
# [cfg (feature = "postgres_backend")] impl ToSql < sql_types :: Oid , Pg > for u32 { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Pg >) -> serialize :: Result { out . write_u32 :: < NetworkEndian > (* self) . map (| _ | IsNull :: No) . map_err (Into :: into) } }
};
}
