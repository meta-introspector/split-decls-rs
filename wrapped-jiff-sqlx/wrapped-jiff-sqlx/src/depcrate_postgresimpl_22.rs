// Generated macro for impl_22 (impl)
macro_rules! Depcrate_postgresimpl_22 {
() => {
// Module: crate::postgres
// Provides: {"impl_22"}
// Dependencies: {}
impl Encode < '_ , Postgres > for Date { fn encode_by_ref (& self , buf : & mut PgArgumentBuffer ,) -> Result < IsNull , BoxDynError > { let days = (self . to_jiff () - POSTGRES_EPOCH_DATE) . get_days () ; Encode :: < Postgres > :: encode (days , buf) } }
};
}
