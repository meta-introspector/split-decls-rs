// Generated macro for impl_99 (impl)
macro_rules! Depcrate_format_args_format_argimpl_99 {
() => {
// Module: crate::format_args::format_arg
// Provides: {"impl_99"}
// Dependencies: {}
impl Parse for FormatArg { fn parse (input : ParseStream) -> Result < Self > { let arg_name : Option < (Ident , token :: Eq) > = if input . peek2 (Token ! [=]) { Some ((input . parse () ? , input . parse () ?)) } else { None } ; let expr : Expr = input . parse () ? ; Ok (FormatArg { arg_name , expr }) } }
};
}
