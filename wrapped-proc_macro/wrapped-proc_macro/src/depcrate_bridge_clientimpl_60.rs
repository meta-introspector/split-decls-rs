// Generated macro for impl_60 (impl)
macro_rules! Depcrate_bridge_clientimpl_60 {
() => {
// Module: crate::bridge::client
// Provides: {"impl_60"}
// Dependencies: {}
impl Client < (crate :: TokenStream , crate :: TokenStream) , crate :: TokenStream > { pub const fn expand2 (f : impl Fn (crate :: TokenStream , crate :: TokenStream) -> crate :: TokenStream + Copy ,) -> Self { Client { handle_counters : & COUNTERS , run : super :: selfless_reify :: reify_to_extern_c_fn_hrt_bridge (move | bridge | { run_client (bridge , | (input , input2) | { f (crate :: TokenStream (Some (input)) , crate :: TokenStream (Some (input2))) . 0 }) }) , _marker : PhantomData , } } }
};
}
