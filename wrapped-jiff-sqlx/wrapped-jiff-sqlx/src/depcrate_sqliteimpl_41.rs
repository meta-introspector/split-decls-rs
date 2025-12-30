// Generated macro for impl_41 (impl)
macro_rules! Depcrate_sqliteimpl_41 {
() => {
// Module: crate::sqlite
// Provides: {"impl_41"}
// Dependencies: {}
impl Encode < '_ , Sqlite > for DateTime { fn encode_by_ref (& self , buf : & mut Vec < SqliteArgumentValue < '_ > > ,) -> Result < IsNull , BoxDynError > { Encode :: < Sqlite > :: encode (self . to_jiff () . to_string () , buf) } }
};
}
