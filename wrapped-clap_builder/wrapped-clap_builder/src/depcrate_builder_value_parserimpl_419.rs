// Generated macro for impl_419 (impl)
macro_rules! Depcrate_builder_value_parserimpl_419 {
() => {
// Module: crate::builder::value_parser
// Provides: {"impl_419"}
// Dependencies: {}
impl UnknownArgumentValueParser { # [doc = " Suggest an alternative argument"] pub fn suggest_arg (arg : impl Into < Str >) -> Self { Self { arg : Some (arg . into ()) , suggestions : Default :: default () , } } # [doc = " Provide a general suggestion"] pub fn suggest (text : impl Into < StyledStr >) -> Self { Self { arg : Default :: default () , suggestions : vec ! [text . into ()] , } } # [doc = " Extend the suggestions"] pub fn and_suggest (mut self , text : impl Into < StyledStr >) -> Self { self . suggestions . push (text . into ()) ; self } }
};
}
