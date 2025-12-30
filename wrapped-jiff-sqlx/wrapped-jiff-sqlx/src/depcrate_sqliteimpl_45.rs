// Generated macro for impl_45 (impl)
macro_rules! Depcrate_sqliteimpl_45 {
() => {
// Module: crate::sqlite
// Provides: {"impl_45"}
// Dependencies: {}
impl < 'r > Decode < 'r , Sqlite > for Date { fn decode (value : SqliteValueRef < 'r >) -> Result < Self , BoxDynError > { let text = < & [u8] as Decode < Sqlite > > :: decode (value) ? ; let date = PARSER . parse_date (text) ? ; Ok (date . to_sqlx ()) } }
};
}
