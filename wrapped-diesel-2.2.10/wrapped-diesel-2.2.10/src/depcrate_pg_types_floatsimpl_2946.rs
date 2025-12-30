// Generated macro for impl_2946 (impl)
macro_rules! Depcrate_pg_types_floatsimpl_2946 {
() => {
// Module: crate::pg::types::floats
// Provides: {"impl_2946"}
// Dependencies: {}
# [cfg (feature = "postgres_backend")] impl ToSql < sql_types :: Float , Pg > for f32 { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Pg >) -> serialize :: Result { out . write_f32 :: < NetworkEndian > (* self) . map (| _ | IsNull :: No) . map_err (| e | Box :: new (e) as Box < dyn Error + Send + Sync >) } }
};
}
