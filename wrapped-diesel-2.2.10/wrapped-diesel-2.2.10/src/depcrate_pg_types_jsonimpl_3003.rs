// Generated macro for impl_3003 (impl)
macro_rules! Depcrate_pg_types_jsonimpl_3003 {
() => {
// Module: crate::pg::types::json
// Provides: {"impl_3003"}
// Dependencies: {}
# [cfg (all (feature = "postgres_backend" , feature = "serde_json"))] impl ToSql < sql_types :: Jsonb , Pg > for serde_json :: Value { fn to_sql < 'b > (& 'b self , out : & mut Output < 'b , '_ , Pg >) -> serialize :: Result { out . write_all (& [1]) ? ; serde_json :: to_writer (out , self) . map (| _ | IsNull :: No) . map_err (Into :: into) } }
};
}
