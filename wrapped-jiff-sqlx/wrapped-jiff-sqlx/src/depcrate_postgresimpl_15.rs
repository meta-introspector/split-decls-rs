// Generated macro for impl_15 (impl)
macro_rules! Depcrate_postgresimpl_15 {
() => {
// Module: crate::postgres
// Provides: {"impl_15"}
// Dependencies: {}
impl < 'r > Decode < 'r , Postgres > for Timestamp { fn decode (value : PgValueRef < 'r >) -> Result < Timestamp , BoxDynError > { match value . format () { PgValueFormat :: Binary => { let micros : i64 = Decode :: < Postgres > :: decode (value) ? ; let micros = jiff :: SignedDuration :: from_micros (micros) ; let epoch = jiff :: Timestamp :: from_second (POSTGRES_EPOCH_TIMESTAMP) . unwrap () ; Ok (epoch . checked_add (micros) ? . to_sqlx ()) } PgValueFormat :: Text => { Ok (value . as_str () ? . parse :: < jiff :: Timestamp > () ? . to_sqlx ()) } } } }
};
}
