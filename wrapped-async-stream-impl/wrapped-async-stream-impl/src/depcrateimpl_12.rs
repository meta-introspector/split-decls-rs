// Generated macro for impl_12 (impl)
macro_rules! Depcrateimpl_12 {
() => {
// Module: crate
// Provides: {"impl_12"}
// Dependencies: {}
impl Scrub < '_ > { fn visit_token_stream (& mut self , tokens : & mut TokenStream2) -> bool { let (mut out , mut modified) = (TokenStream2 :: new () , false) ; visit_token_stream_impl (self , tokens . clone () , & mut modified , & mut out) ; if modified { * tokens = out ; } modified } }
};
}
