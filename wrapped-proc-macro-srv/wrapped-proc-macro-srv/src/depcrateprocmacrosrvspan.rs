// Generated macro for ProcMacroSrvSpan (trait)
macro_rules! DepcrateProcMacroSrvSpan {
() => {
// Module: crate
// Provides: {"ProcMacroSrvSpan"}
// Dependencies: {}
pub trait ProcMacroSrvSpan : Copy + Send { type Server : proc_macro :: bridge :: server :: Server < TokenStream = TokenStream < Self > > ; fn make_server (call_site : Self , def_site : Self , mixed_site : Self) -> Self :: Server ; }
};
}
