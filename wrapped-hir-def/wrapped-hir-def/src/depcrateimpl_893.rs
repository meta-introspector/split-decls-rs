// Generated macro for impl_893 (impl)
macro_rules! Depcrateimpl_893 {
() => {
// Module: crate
// Provides: {"impl_893"}
// Dependencies: {}
impl MacroId { pub fn is_attribute (self , db : & dyn DefDatabase) -> bool { matches ! (self , MacroId :: ProcMacroId (it) if it . lookup (db) . kind == ProcMacroKind :: Attr) } }
};
}
