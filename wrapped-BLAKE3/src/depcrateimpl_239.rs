// Generated macro for impl_239 (impl)
macro_rules! Depcrateimpl_239 {
() => {
// Module: crate
// Provides: {"impl_239"}
// Dependencies: {}
# [cfg (feature = "zeroize")] impl Zeroize for OutputReader { fn zeroize (& mut self) { let Self { inner , position_within_block , } = self ; inner . zeroize () ; position_within_block . zeroize () ; } }
};
}
