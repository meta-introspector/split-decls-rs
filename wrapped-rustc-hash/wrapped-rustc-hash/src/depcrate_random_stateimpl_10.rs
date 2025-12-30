// Generated macro for impl_10 (impl)
macro_rules! Depcrate_random_stateimpl_10 {
() => {
// Module: crate::random_state
// Provides: {"impl_10"}
// Dependencies: {}
impl core :: hash :: BuildHasher for FxRandomState { type Hasher = FxHasher ; fn build_hasher (& self) -> Self :: Hasher { FxHasher :: with_seed (self . seed) } }
};
}
