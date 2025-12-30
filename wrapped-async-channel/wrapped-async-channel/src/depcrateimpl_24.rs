// Generated macro for impl_24 (impl)
macro_rules! Depcrateimpl_24 {
() => {
// Module: crate
// Provides: {"impl_24"}
// Dependencies: {}
impl < T > Clone for Sender < T > { fn clone (& self) -> Sender < T > { let count = self . channel . sender_count . fetch_add (1 , Ordering :: Relaxed) ; if count > usize :: MAX / 2 { abort () ; } Sender { channel : self . channel . clone () , } } }
};
}
