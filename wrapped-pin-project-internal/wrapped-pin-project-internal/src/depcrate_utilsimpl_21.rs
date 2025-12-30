// Generated macro for impl_21 (impl)
macro_rules! Depcrate_utilsimpl_21 {
() => {
// Module: crate::utils
// Provides: {"impl_21"}
// Dependencies: {}
impl < 'a > ParseBufferExt < 'a > for ParseBuffer < 'a > { fn parenthesized (self) -> Result < ParseBuffer < 'a > > { let content ; let _ : token :: Paren = syn :: parenthesized ! (content in self) ; Ok (content) } }
};
}
