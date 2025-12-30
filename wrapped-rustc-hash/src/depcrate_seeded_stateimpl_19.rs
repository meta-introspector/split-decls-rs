// Generated macro for impl_19 (impl)
macro_rules! Depcrate_seeded_stateimpl_19 {
() => {
// Module: crate::seeded_state
// Provides: {"impl_19"}
// Dependencies: {}
impl core :: hash :: BuildHasher for FxSeededState { type Hasher = FxHasher ; fn build_hasher (& self) -> Self :: Hasher { FxHasher :: with_seed (self . seed) } }
};
}
