// Generated macro for impl_27 (impl)
macro_rules! Depcrate_postgresimpl_27 {
() => {
// Module: crate::postgres
// Provides: {"impl_27"}
// Dependencies: {}
impl < 'r > Decode < 'r , Postgres > for Time { fn decode (value : PgValueRef < 'r >) -> Result < Self , BoxDynError > { match value . format () { PgValueFormat :: Binary => { let micros : i64 = Decode :: < Postgres > :: decode (value) ? ; let micros = jiff :: SignedDuration :: from_micros (micros) ; Ok (MIDNIGHT . checked_add (micros) ? . to_sqlx ()) } PgValueFormat :: Text => { Ok (value . as_str () ? . parse :: < civil :: Time > () ? . to_sqlx ()) } } } }
};
}
