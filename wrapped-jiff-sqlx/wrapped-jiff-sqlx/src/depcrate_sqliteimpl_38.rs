// Generated macro for impl_38 (impl)
macro_rules! Depcrate_sqliteimpl_38 {
() => {
// Module: crate::sqlite
// Provides: {"impl_38"}
// Dependencies: {}
impl Encode < '_ , Sqlite > for Timestamp { fn encode_by_ref (& self , buf : & mut Vec < SqliteArgumentValue < '_ > > ,) -> Result < IsNull , BoxDynError > { Encode :: < Sqlite > :: encode (self . to_jiff () . to_string () , buf) } }
};
}
