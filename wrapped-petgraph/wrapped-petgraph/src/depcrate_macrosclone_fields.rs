// Generated macro for clone_fields (macro)
macro_rules! Depcrate_macrosclone_fields {
() => {
// Module: crate::macros
// Provides: {"clone_fields"}
// Dependencies: {}
macro_rules ! clone_fields { ($ name : ident , $ ($ field : ident) ,+ $ (,) *) => (fn clone (& self) -> Self { $ name { $ ($ field : self . $ field . clone ()) ,* } }) ; }
};
}
