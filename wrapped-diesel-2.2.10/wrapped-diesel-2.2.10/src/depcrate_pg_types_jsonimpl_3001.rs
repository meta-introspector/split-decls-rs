// Generated macro for impl_3001 (impl)
macro_rules! Depcrate_pg_types_jsonimpl_3001 {
() => {
// Module: crate::pg::types::json
// Provides: {"impl_3001"}
// Dependencies: {}
# [cfg (all (feature = "postgres_backend" , feature = "serde_json"))] impl ToSql < sql_types :: Json , Pg > for serde_json :: Value { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Pg >) -> serialize :: Result { serde_json :: to_writer (out , self) . map (| _ | IsNull :: No) . map_err (Into :: into) } }
};
}
