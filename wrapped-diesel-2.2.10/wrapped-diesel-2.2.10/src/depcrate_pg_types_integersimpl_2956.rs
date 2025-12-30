// Generated macro for impl_2956 (impl)
macro_rules! Depcrate_pg_types_integersimpl_2956 {
() => {
// Module: crate::pg::types::integers
// Provides: {"impl_2956"}
// Dependencies: {}
# [cfg (feature = "postgres_backend")] impl FromSql < sql_types :: SmallInt , Pg > for i16 { # [inline (always)] fn from_sql (value : PgValue < '_ >) -> deserialize :: Result < Self > { let mut bytes = value . as_bytes () ; if bytes . len () < 2 { return emit_size_error ("Received less than 2 bytes while decoding an i16. \
                    Was an expression of a different type accidentally marked as SmallInt?" ,) ; } if bytes . len () > 2 { return emit_size_error ("Received more than 2 bytes while decoding an i16. \
                    Was an Integer expression accidentally marked as SmallInt?" ,) ; } bytes . read_i16 :: < NetworkEndian > () . map_err (| e | Box :: new (e) as Box < _ >) } }
};
}
