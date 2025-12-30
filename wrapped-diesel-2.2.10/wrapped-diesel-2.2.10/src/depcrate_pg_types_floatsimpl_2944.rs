// Generated macro for impl_2944 (impl)
macro_rules! Depcrate_pg_types_floatsimpl_2944 {
() => {
// Module: crate::pg::types::floats
// Provides: {"impl_2944"}
// Dependencies: {}
# [cfg (feature = "postgres_backend")] impl FromSql < sql_types :: Float , Pg > for f32 { fn from_sql (value : PgValue < '_ >) -> deserialize :: Result < Self > { let mut bytes = value . as_bytes () ; if bytes . len () < 4 { return deserialize :: Result :: Err ("Received less than 4 bytes while decoding an f32. \
                 Was a numeric accidentally marked as float?" . into () ,) ; } if bytes . len () > 4 { return deserialize :: Result :: Err ("Received more than 4 bytes while decoding an f32. \
                 Was a double accidentally marked as float?" . into () ,) ; } bytes . read_f32 :: < NetworkEndian > () . map_err (| e | Box :: new (e) as Box < dyn Error + Send + Sync >) } }
};
}
