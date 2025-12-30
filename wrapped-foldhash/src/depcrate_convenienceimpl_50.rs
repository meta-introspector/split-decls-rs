// Generated macro for impl_50 (impl)
macro_rules! Depcrate_convenienceimpl_50 {
() => {
// Module: crate::convenience
// Provides: {"impl_50"}
// Dependencies: {}
impl < K , V > HashMapExt for std :: collections :: HashMap < K , V , RandomState > { # [inline (always)] fn new () -> Self { Self :: with_hasher (RandomState :: default ()) } # [inline (always)] fn with_capacity (capacity : usize) -> Self { Self :: with_capacity_and_hasher (capacity , RandomState :: default ()) } }
};
}
