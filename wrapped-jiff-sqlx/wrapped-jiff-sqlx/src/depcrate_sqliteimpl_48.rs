// Generated macro for impl_48 (impl)
macro_rules! Depcrate_sqliteimpl_48 {
() => {
// Module: crate::sqlite
// Provides: {"impl_48"}
// Dependencies: {}
impl < 'r > Decode < 'r , Sqlite > for Time { fn decode (value : SqliteValueRef < 'r >) -> Result < Self , BoxDynError > { let text = < & [u8] as Decode < Sqlite > > :: decode (value) ? ; let date = PARSER . parse_time (text) ? ; Ok (date . to_sqlx ()) } }
};
}
