// Generated macro for impl_51 (impl)
macro_rules! Depcrate_convenienceimpl_51 {
() => {
// Module: crate::convenience
// Provides: {"impl_51"}
// Dependencies: {}
impl < K , V > HashMapExt for std :: collections :: HashMap < K , V , FixedState > { # [inline (always)] fn new () -> Self { Self :: with_hasher (FixedState :: default ()) } # [inline (always)] fn with_capacity (capacity : usize) -> Self { Self :: with_capacity_and_hasher (capacity , FixedState :: default ()) } }
};
}
