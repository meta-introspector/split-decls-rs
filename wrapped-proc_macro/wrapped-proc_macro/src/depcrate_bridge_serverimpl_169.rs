// Generated macro for impl_169 (impl)
macro_rules! Depcrate_bridge_serverimpl_169 {
() => {
// Module: crate::bridge::server
// Provides: {"impl_169"}
// Dependencies: {}
impl client :: Client < crate :: TokenStream , crate :: TokenStream > { pub fn run < S > (& self , strategy : & impl ExecutionStrategy , server : S , input : S :: TokenStream , force_show_panics : bool ,) -> Result < S :: TokenStream , PanicMessage > where S : Server , S :: TokenStream : Default , { let client :: Client { handle_counters , run , _marker } = * self ; run_server (strategy , handle_counters , server , < MarkedTypes < S > as Types > :: TokenStream :: mark (input) , run , force_show_panics ,) . map (| s | < Option < < MarkedTypes < S > as Types > :: TokenStream > > :: unmark (s) . unwrap_or_default ()) } }
};
}
