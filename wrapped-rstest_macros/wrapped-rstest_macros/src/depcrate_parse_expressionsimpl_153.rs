// Generated macro for impl_153 (impl)
macro_rules! Depcrate_parse_expressionsimpl_153 {
() => {
// Module: crate::parse::expressions
// Provides: {"impl_153"}
// Dependencies: {}
impl Parse for Expressions { fn parse (input : ParseStream) -> Result < Self > { let values = input . parse_terminated (Parse :: parse , Token ! [,]) ? . into_iter () . collect () ; Ok (Self (values)) } }
};
}
