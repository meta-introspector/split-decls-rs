// Generated macro for Client (struct)
macro_rules! Depcrate_bridge_clientClient {
() => {
// Module: crate::bridge::client
// Provides: {"Client"}
// Dependencies: {}
# [doc = " A client-side RPC entry-point, which may be using a different `proc_macro`"] # [doc = " from the one used by the server, but can be invoked compatibly."] # [doc = ""] # [doc = " Note that the (phantom) `I` (\"input\") and `O` (\"output\") type parameters"] # [doc = " decorate the `Client<I, O>` with the RPC \"interface\" of the entry-point, but"] # [doc = " do not themselves participate in ABI, at all, only facilitate type-checking."] # [doc = ""] # [doc = " E.g. `Client<TokenStream, TokenStream>` is the common proc macro interface,"] # [doc = " used for `#[proc_macro] fn foo(input: TokenStream) -> TokenStream`,"] # [doc = " indicating that the RPC input and output will be serialized token streams,"] # [doc = " and forcing the use of APIs that take/return `S::TokenStream`, server-side."] # [repr (C)] pub struct Client < I , O > { pub (super) handle_counters : & 'static HandleCounters , pub (super) run : extern "C" fn (BridgeConfig < '_ >) -> Buffer , pub (super) _marker : PhantomData < fn (I) -> O > , }
};
}
