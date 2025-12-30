// Generated macro for impl_12 (impl)
macro_rules! Depcrate_appimpl_12 {
() => {
// Module: crate::app
// Provides: {"impl_12"}
// Dependencies: {}
impl RandomSignal { pub fn new (lower : u64 , upper : u64) -> Self { Self { distribution : Uniform :: new (lower , upper) . expect ("invalid range") , rng : rand :: rng () , } } }
};
}
