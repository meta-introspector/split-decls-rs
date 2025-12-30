// Generated macro for impl_2962 (impl)
macro_rules! Depcrate_pg_types_integersimpl_2962 {
() => {
// Module: crate::pg::types::integers
// Provides: {"impl_2962"}
// Dependencies: {}
# [cfg (feature = "postgres_backend")] impl ToSql < sql_types :: BigInt , Pg > for i64 { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Pg >) -> serialize :: Result { out . write_i64 :: < NetworkEndian > (* self) . map (| _ | IsNull :: No) . map_err (| e | Box :: new (e) as Box < _ >) } }
};
}
