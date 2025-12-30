// Generated macro for impl_2957 (impl)
macro_rules! Depcrate_pg_types_integersimpl_2957 {
() => {
// Module: crate::pg::types::integers
// Provides: {"impl_2957"}
// Dependencies: {}
# [cfg (feature = "postgres_backend")] impl FromSql < sql_types :: Integer , Pg > for i32 { # [inline (always)] fn from_sql (value : PgValue < '_ >) -> deserialize :: Result < Self > { let mut bytes = value . as_bytes () ; if bytes . len () < 4 { return emit_size_error ("Received less than 4 bytes while decoding an i32. \
                    Was an SmallInt expression accidentally marked as Integer?" ,) ; } if bytes . len () > 4 { return emit_size_error ("Received more than 4 bytes while decoding an i32. \
                    Was an BigInt expression accidentally marked as Integer?" ,) ; } bytes . read_i32 :: < NetworkEndian > () . map_err (| e | Box :: new (e) as Box < _ >) } }
};
}
