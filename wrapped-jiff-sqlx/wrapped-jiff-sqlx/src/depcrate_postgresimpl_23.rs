// Generated macro for impl_23 (impl)
macro_rules! Depcrate_postgresimpl_23 {
() => {
// Module: crate::postgres
// Provides: {"impl_23"}
// Dependencies: {}
impl < 'r > Decode < 'r , Postgres > for Date { fn decode (value : PgValueRef < 'r >) -> Result < Date , BoxDynError > { match value . format () { PgValueFormat :: Binary => { let days : i32 = Decode :: < Postgres > :: decode (value) ? ; let span = jiff :: Span :: new () . try_days (days) ? ; Ok (POSTGRES_EPOCH_DATE . checked_add (span) ? . to_sqlx ()) } PgValueFormat :: Text => { Ok (value . as_str () ? . parse :: < civil :: Date > () ? . to_sqlx ()) } } } }
};
}
