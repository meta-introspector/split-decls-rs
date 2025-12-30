// Generated macro for impl_54 (impl)
macro_rules! Depcrate_convenienceimpl_54 {
() => {
// Module: crate::convenience
// Provides: {"impl_54"}
// Dependencies: {}
impl < T > HashSetExt for std :: collections :: HashSet < T , FixedState > { # [inline (always)] fn new () -> Self { Self :: with_hasher (FixedState :: default ()) } # [inline (always)] fn with_capacity (capacity : usize) -> Self { Self :: with_capacity_and_hasher (capacity , FixedState :: default ()) } }
};
}
