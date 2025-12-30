// Generated macro for impl_435 (impl)
macro_rules! Depcrate_render_apply_argumentsimpl_435 {
() => {
// Module: crate::render::apply_arguments
// Provides: {"impl_435"}
// Dependencies: {}
impl ApplyArguments for ItemFn { type Output = () ; type Context = () ; fn apply_arguments (& mut self , arguments : & mut ArgumentsInfo , _ : & mut ()) { let args = self . sig . inputs . iter () . cloned () . collect :: < Vec < _ > > () ; self . sig . apply_arguments (arguments , & mut ()) ; let rebound_awaited_args = args . iter () . filter_map (MaybePat :: maybe_pat) . filter (| p | arguments . is_future_await (p)) . filter_map (MaybePatIdent :: maybe_patident) . map (| p | { let a = & p . ident ; quote :: quote ! { let # p = # a . await ; } }) ; let orig_block_impl = self . block . clone () ; self . block = parse_quote ! { { # (# rebound_awaited_args) * # orig_block_impl } } ; } }
};
}
