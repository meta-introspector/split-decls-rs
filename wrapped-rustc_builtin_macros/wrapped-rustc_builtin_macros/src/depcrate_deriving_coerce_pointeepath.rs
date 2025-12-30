// Generated macro for path (macro)
macro_rules! Depcrate_deriving_coerce_pointeepath {
() => {
// Module: crate::deriving::coerce_pointee
// Provides: {"path"}
// Dependencies: {}
macro_rules ! path { ($ span : expr , $ ($ part : ident) ::*) => { vec ! [$ (Ident :: new (sym ::$ part , $ span) ,) *] } }
};
}
