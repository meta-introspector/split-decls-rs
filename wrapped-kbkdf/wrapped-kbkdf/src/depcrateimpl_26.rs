// Generated macro for impl_26 (impl)
macro_rules! Depcrateimpl_26 {
() => {
// Module: crate
// Provides: {"impl_26"}
// Dependencies: {}
impl < 'a , Prf , K , R > Feedback < 'a , Prf , K , R > where Prf : Mac , { # [doc = " Creates a new [`Feedback`] instance with an optional IV."] pub fn new (iv : Option < & 'a Array < u8 , Prf :: OutputSize > >) -> Self { Self { iv , _marker : PhantomData , } } }
};
}
