// Generated macro for impl_19 (impl)
macro_rules! Depcrate_postgresimpl_19 {
() => {
// Module: crate::postgres
// Provides: {"impl_19"}
// Dependencies: {}
impl < 'r > Decode < 'r , Postgres > for DateTime { fn decode (value : PgValueRef < 'r >) -> Result < DateTime , BoxDynError > { match value . format () { PgValueFormat :: Binary => { let micros : i64 = Decode :: < Postgres > :: decode (value) ? ; let micros = jiff :: SignedDuration :: from_micros (micros) ; Ok (POSTGRES_EPOCH_DATETIME . checked_add (micros) ? . to_sqlx ()) } PgValueFormat :: Text => { Ok (value . as_str () ? . parse :: < civil :: DateTime > () ? . to_sqlx ()) } } } }
};
}
