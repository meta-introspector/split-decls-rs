// Generated macro for impl_166 (impl)
macro_rules! Depcrate_unrawimpl_166 {
() => {
// Module: crate::unraw
// Provides: {"impl_166"}
// Dependencies: {}
impl Parse for IdentUnraw { fn parse (input : ParseStream) -> Result < Self > { input . call (Ident :: parse_any) . map (IdentUnraw :: new) } }
};
}
