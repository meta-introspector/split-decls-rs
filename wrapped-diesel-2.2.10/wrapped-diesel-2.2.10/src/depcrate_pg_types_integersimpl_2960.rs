// Generated macro for impl_2960 (impl)
macro_rules! Depcrate_pg_types_integersimpl_2960 {
() => {
// Module: crate::pg::types::integers
// Provides: {"impl_2960"}
// Dependencies: {}
# [cfg (feature = "postgres_backend")] impl ToSql < sql_types :: SmallInt , Pg > for i16 { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Pg >) -> serialize :: Result { out . write_i16 :: < NetworkEndian > (* self) . map (| _ | IsNull :: No) . map_err (| e | Box :: new (e) as Box < _ >) } }
};
}
