// Generated macro for impl_2959 (impl)
macro_rules! Depcrate_pg_types_integersimpl_2959 {
() => {
// Module: crate::pg::types::integers
// Provides: {"impl_2959"}
// Dependencies: {}
# [cfg (feature = "postgres_backend")] impl FromSql < sql_types :: BigInt , Pg > for i64 { # [inline (always)] fn from_sql (value : PgValue < '_ >) -> deserialize :: Result < Self > { let mut bytes = value . as_bytes () ; if bytes . len () < 8 { return emit_size_error ("Received less than 8 bytes while decoding an i64. \
                    Was an Integer expression accidentally marked as BigInt?" ,) ; } if bytes . len () > 8 { return emit_size_error ("Received more than 8 bytes while decoding an i64. \
                    Was an expression of a different type expression accidentally marked as BigInt?") ; } bytes . read_i64 :: < NetworkEndian > () . map_err (| e | Box :: new (e) as Box < _ >) } }
};
}
