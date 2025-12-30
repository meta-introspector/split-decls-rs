// Generated macro for impl_18 (impl)
macro_rules! Depcrate_postgresimpl_18 {
() => {
// Module: crate::postgres
// Provides: {"impl_18"}
// Dependencies: {}
impl Encode < '_ , Postgres > for DateTime { fn encode_by_ref (& self , buf : & mut PgArgumentBuffer ,) -> Result < IsNull , BoxDynError > { let micros = self . to_jiff () . duration_since (POSTGRES_EPOCH_DATETIME) . as_micros () ; let micros = i64 :: try_from (micros) . unwrap () ; Encode :: < Postgres > :: encode (micros , buf) } }
};
}
