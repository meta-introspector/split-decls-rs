// Generated macro for impl_42 (impl)
macro_rules! Depcrate_sqliteimpl_42 {
() => {
// Module: crate::sqlite
// Provides: {"impl_42"}
// Dependencies: {}
impl < 'r > Decode < 'r , Sqlite > for DateTime { fn decode (value : SqliteValueRef < 'r >) -> Result < Self , BoxDynError > { let text = < & [u8] as Decode < Sqlite > > :: decode (value) ? ; let date = PARSER . parse_datetime (text) ? ; Ok (date . to_sqlx ()) } }
};
}
