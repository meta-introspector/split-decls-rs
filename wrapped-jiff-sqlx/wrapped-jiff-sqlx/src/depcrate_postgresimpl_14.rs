// Generated macro for impl_14 (impl)
macro_rules! Depcrate_postgresimpl_14 {
() => {
// Module: crate::postgres
// Provides: {"impl_14"}
// Dependencies: {}
impl Encode < '_ , Postgres > for Timestamp { fn encode_by_ref (& self , buf : & mut PgArgumentBuffer ,) -> Result < IsNull , BoxDynError > { let dt = UTC . to_datetime (self . to_jiff ()) . to_sqlx () ; Encode :: < Postgres > :: encode (dt , buf) } }
};
}
