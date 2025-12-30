// Generated macro for impl_2945 (impl)
macro_rules! Depcrate_pg_types_floatsimpl_2945 {
() => {
// Module: crate::pg::types::floats
// Provides: {"impl_2945"}
// Dependencies: {}
# [cfg (feature = "postgres_backend")] impl FromSql < sql_types :: Double , Pg > for f64 { fn from_sql (value : PgValue < '_ >) -> deserialize :: Result < Self > { let mut bytes = value . as_bytes () ; if bytes . len () < 8 { return deserialize :: Result :: Err ("Received less than 8 bytes while decoding an f64. \
                    Was a float accidentally marked as double?" . into () ,) ; } if bytes . len () > 8 { return deserialize :: Result :: Err ("Received more than 8 bytes while decoding an f64. \
                    Was a numeric accidentally marked as double?" . into () ,) ; } bytes . read_f64 :: < NetworkEndian > () . map_err (| e | Box :: new (e) as Box < dyn Error + Send + Sync >) } }
};
}
