// Generated macro for parse_attribute (function)
macro_rules! Depcrate_pin_project_attributeparse_attribute {
() => {
// Module: crate::pin_project::attribute
// Provides: {"parse_attribute"}
// Dependencies: {}
pub (super) fn parse_attribute (args : & TokenStream , input : TokenStream) -> Result < TokenStream > { let Input { attrs , body } = syn :: parse2 (input) ? ; Ok (quote ! { # (# attrs) * # [derive (:: pin_project :: __private :: __PinProjectInternalDerive)] # [pin (__private (# args))] # body }) }
};
}
