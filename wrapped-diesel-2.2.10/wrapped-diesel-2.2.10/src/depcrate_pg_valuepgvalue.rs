// Generated macro for PgValue (struct)
macro_rules! Depcrate_pg_valuePgValue {
() => {
// Module: crate::pg::value
// Provides: {"PgValue"}
// Dependencies: {}
# [doc = " Raw postgres value as received from the database"] # [derive (Clone , Copy)] # [allow (missing_debug_implementations)] # [cfg (feature = "postgres_backend")] pub struct PgValue < 'a > { raw_value : & 'a [u8] , type_oid_lookup : & 'a dyn TypeOidLookup , }
};
}
