// Generated macro for impl_27 (impl)
macro_rules! Depcrateimpl_27 {
() => {
// Module: crate
// Provides: {"impl_27"}
// Dependencies: {}
impl < K , V > LruEntry < K , V > { fn new (key : K , val : V) -> Self { LruEntry { key : mem :: MaybeUninit :: new (key) , val : mem :: MaybeUninit :: new (val) , prev : ptr :: null_mut () , next : ptr :: null_mut () , } } fn new_sigil () -> Self { LruEntry { key : mem :: MaybeUninit :: uninit () , val : mem :: MaybeUninit :: uninit () , prev : ptr :: null_mut () , next : ptr :: null_mut () , } } }
};
}
