// Generated macro for impl_30 (impl)
macro_rules! Depcrate_postgresimpl_30 {
() => {
// Module: crate::postgres
// Provides: {"impl_30"}
// Dependencies: {}
impl < 'r > Decode < 'r , Postgres > for Span { fn decode (value : PgValueRef < 'r >) -> Result < Self , BoxDynError > { let interval : PgInterval = Decode :: < Postgres > :: decode (value) ? ; let span = jiff :: Span :: new () . try_months (interval . months) ? . try_days (interval . days) ? . try_microseconds (interval . microseconds) ? ; Ok (span . to_sqlx ()) } }
};
}
