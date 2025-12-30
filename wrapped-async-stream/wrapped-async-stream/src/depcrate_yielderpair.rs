// Generated macro for pair (function)
macro_rules! Depcrate_yielderpair {
() => {
// Module: crate::yielder
// Provides: {"pair"}
// Dependencies: {}
# [doc (hidden)] pub unsafe fn pair < T > () -> (Sender < T > , Receiver < T >) { let tx = Sender { _p : PhantomData } ; let rx = Receiver { _p : PhantomData } ; (tx , rx) }
};
}
