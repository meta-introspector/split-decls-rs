// Generated macro for impl_20 (impl)
macro_rules! Depcrate_utilsimpl_20 {
() => {
// Module: crate::utils
// Provides: {"impl_20"}
// Dependencies: {}
impl < 'a > ParseBufferExt < 'a > for ParseStream < 'a > { fn parenthesized (self) -> Result < ParseBuffer < 'a > > { let content ; let _ : token :: Paren = syn :: parenthesized ! (content in self) ; Ok (content) } }
};
}
