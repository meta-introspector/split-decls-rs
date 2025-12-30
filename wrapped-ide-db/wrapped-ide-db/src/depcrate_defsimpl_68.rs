// Generated macro for impl_68 (impl)
macro_rules! Depcrate_defsimpl_68 {
() => {
// Module: crate::defs
// Provides: {"impl_68"}
// Dependencies: {}
impl From < DocLinkDef > for Definition { fn from (def : DocLinkDef) -> Self { match def { DocLinkDef :: ModuleDef (it) => it . into () , DocLinkDef :: Field (it) => it . into () , DocLinkDef :: SelfType (it) => it . into () , } } }
};
}
