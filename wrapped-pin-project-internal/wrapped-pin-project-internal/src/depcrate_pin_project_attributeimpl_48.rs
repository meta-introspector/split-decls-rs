// Generated macro for impl_48 (impl)
macro_rules! Depcrate_pin_project_attributeimpl_48 {
() => {
// Module: crate::pin_project::attribute
// Provides: {"impl_48"}
// Dependencies: {}
impl Parse for Input { fn parse (input : ParseStream < '_ >) -> Result < Self > { let attrs = input . call (Attribute :: parse_outer) ? ; let ahead = input . fork () ; let _vis : Visibility = ahead . parse () ? ; if ! ahead . peek (Token ! [struct]) && ! ahead . peek (Token ! [enum]) { bail ! (input . parse ::< TokenStream > () ?, "#[pin_project] attribute may only be used on structs or enums") ; } else if let Some (attr) = attrs . find (PIN) { bail ! (attr , "#[pin] attribute may only be used on fields of structs or variants") ; } else if let Some (attr) = attrs . find ("pin_project") { bail ! (attr , "duplicate #[pin_project] attribute") ; } Ok (Self { attrs , body : input . parse () ? }) } }
};
}
