// Generated macro for impl_59 (impl)
macro_rules! Depcrate_bridge_clientimpl_59 {
() => {
// Module: crate::bridge::client
// Provides: {"impl_59"}
// Dependencies: {}
impl Client < crate :: TokenStream , crate :: TokenStream > { pub const fn expand1 (f : impl Fn (crate :: TokenStream) -> crate :: TokenStream + Copy) -> Self { Client { handle_counters : & COUNTERS , run : super :: selfless_reify :: reify_to_extern_c_fn_hrt_bridge (move | bridge | { run_client (bridge , | input | f (crate :: TokenStream (Some (input))) . 0) }) , _marker : PhantomData , } } }
};
}
