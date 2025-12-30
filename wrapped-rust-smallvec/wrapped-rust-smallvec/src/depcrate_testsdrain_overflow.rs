// Generated macro for drain_overflow (function)
macro_rules! Depcrate_testsdrain_overflow {
() => {
// Module: crate::tests
// Provides: {"drain_overflow"}
// Dependencies: {}
# [test] # [should_panic] fn drain_overflow () { let mut v : SmallVec < u8 , 8 > = smallvec ! [0] ; v . drain (..= std :: usize :: MAX) ; }
};
}
