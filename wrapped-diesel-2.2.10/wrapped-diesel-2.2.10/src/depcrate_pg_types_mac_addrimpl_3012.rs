// Generated macro for impl_3012 (impl)
macro_rules! Depcrate_pg_types_mac_addrimpl_3012 {
() => {
// Module: crate::pg::types::mac_addr
// Provides: {"impl_3012"}
// Dependencies: {}
# [cfg (feature = "postgres_backend")] impl FromSql < MacAddr , Pg > for [u8 ; 6] { fn from_sql (value : PgValue < '_ >) -> deserialize :: Result < Self > { value . as_bytes () . try_into () . map_err (| _ | "invalid network address format: input isn't 6 bytes." . into ()) } }
};
}
