// Generated macro for impl_47 (impl)
macro_rules! Depcrate_sqliteimpl_47 {
() => {
// Module: crate::sqlite
// Provides: {"impl_47"}
// Dependencies: {}
impl Encode < '_ , Sqlite > for Time { fn encode_by_ref (& self , buf : & mut Vec < SqliteArgumentValue < '_ > > ,) -> Result < IsNull , BoxDynError > { Encode :: < Sqlite > :: encode (self . to_jiff () . to_string () , buf) } }
};
}
