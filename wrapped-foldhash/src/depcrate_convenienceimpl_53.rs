// Generated macro for impl_53 (impl)
macro_rules! Depcrate_convenienceimpl_53 {
() => {
// Module: crate::convenience
// Provides: {"impl_53"}
// Dependencies: {}
impl < T > HashSetExt for std :: collections :: HashSet < T , RandomState > { # [inline (always)] fn new () -> Self { Self :: with_hasher (RandomState :: default ()) } # [inline (always)] fn with_capacity (capacity : usize) -> Self { Self :: with_capacity_and_hasher (capacity , RandomState :: default ()) } }
};
}
