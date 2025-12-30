// Generated macro for impl_28 (impl)
macro_rules! Depcrateimpl_28 {
() => {
// Module: crate
// Provides: {"impl_28"}
// Dependencies: {}
impl < T > Clone for Receiver < T > { fn clone (& self) -> Receiver < T > { let count = self . channel . receiver_count . fetch_add (1 , Ordering :: Relaxed) ; if count > usize :: MAX / 2 { abort () ; } Receiver { channel : self . channel . clone () , listener : None , _pin : PhantomPinned , } } }
};
}
