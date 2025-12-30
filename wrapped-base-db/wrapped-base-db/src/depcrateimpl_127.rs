// Generated macro for impl_127 (impl)
macro_rules! Depcrateimpl_127 {
() => {
// Module: crate
// Provides: {"impl_127"}
// Dependencies: {}
impl Nonce { # [inline] pub fn new () -> Nonce { Nonce (NEXT_NONCE . fetch_add (1 , std :: sync :: atomic :: Ordering :: SeqCst)) } }
};
}
