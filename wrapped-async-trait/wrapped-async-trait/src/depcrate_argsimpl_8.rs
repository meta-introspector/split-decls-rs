// Generated macro for impl_8 (impl)
macro_rules! Depcrate_argsimpl_8 {
() => {
// Module: crate::args
// Provides: {"impl_8"}
// Dependencies: {}
impl Parse for Args { fn parse (input : ParseStream) -> Result < Self > { match try_parse (input) { Ok (args) if input . is_empty () => Ok (args) , _ => Err (error ()) , } } }
};
}
