// Generated macro for impl_26 (impl)
macro_rules! Depcrate_postgresimpl_26 {
() => {
// Module: crate::postgres
// Provides: {"impl_26"}
// Dependencies: {}
impl Encode < '_ , Postgres > for Time { fn encode_by_ref (& self , buf : & mut PgArgumentBuffer ,) -> Result < IsNull , BoxDynError > { let micros = self . to_jiff () . duration_since (MIDNIGHT) . as_micros () ; let micros = i64 :: try_from (micros) . unwrap () ; Encode :: < Postgres > :: encode (micros , buf) } }
};
}
