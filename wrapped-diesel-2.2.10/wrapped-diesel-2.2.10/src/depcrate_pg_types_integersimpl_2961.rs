// Generated macro for impl_2961 (impl)
macro_rules! Depcrate_pg_types_integersimpl_2961 {
() => {
// Module: crate::pg::types::integers
// Provides: {"impl_2961"}
// Dependencies: {}
# [cfg (feature = "postgres_backend")] impl ToSql < sql_types :: Integer , Pg > for i32 { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Pg >) -> serialize :: Result { out . write_i32 :: < NetworkEndian > (* self) . map (| _ | IsNull :: No) . map_err (| e | Box :: new (e) as Box < _ >) } }
};
}
