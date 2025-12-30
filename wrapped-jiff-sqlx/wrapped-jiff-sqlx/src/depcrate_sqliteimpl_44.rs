// Generated macro for impl_44 (impl)
macro_rules! Depcrate_sqliteimpl_44 {
() => {
// Module: crate::sqlite
// Provides: {"impl_44"}
// Dependencies: {}
impl Encode < '_ , Sqlite > for Date { fn encode_by_ref (& self , buf : & mut Vec < SqliteArgumentValue < '_ > > ,) -> Result < IsNull , BoxDynError > { Encode :: < Sqlite > :: encode (self . to_jiff () . to_string () , buf) } }
};
}
