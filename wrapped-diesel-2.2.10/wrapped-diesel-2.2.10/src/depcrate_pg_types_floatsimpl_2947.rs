// Generated macro for impl_2947 (impl)
macro_rules! Depcrate_pg_types_floatsimpl_2947 {
() => {
// Module: crate::pg::types::floats
// Provides: {"impl_2947"}
// Dependencies: {}
# [cfg (feature = "postgres_backend")] impl ToSql < sql_types :: Double , Pg > for f64 { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Pg >) -> serialize :: Result { out . write_f64 :: < NetworkEndian > (* self) . map (| _ | IsNull :: No) . map_err (| e | Box :: new (e) as Box < dyn Error + Send + Sync >) } }
};
}
