// Generated macro for impl_2933 (impl)
macro_rules! Depcrate_ctxhashimpl_2933 {
() => {
// Module: crate::ctxhash
// Provides: {"impl_2933"}
// Dependencies: {}
impl < 'a , K , V > VacantEntry < 'a , K , V > { # [doc = " Insert a new value."] pub fn insert (self , v : V) { self . raw . insert (BucketData { hash : self . hash , k : self . key , v , }) ; } }
};
}
