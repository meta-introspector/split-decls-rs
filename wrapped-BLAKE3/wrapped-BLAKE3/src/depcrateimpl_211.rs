// Generated macro for impl_211 (impl)
macro_rules! Depcrateimpl_211 {
() => {
// Module: crate
// Provides: {"impl_211"}
// Dependencies: {}
# [cfg (feature = "zeroize")] impl Zeroize for Output { fn zeroize (& mut self) { let Self { input_chaining_value , block , block_len , counter , flags , platform : _ , } = self ; input_chaining_value . zeroize () ; block . zeroize () ; block_len . zeroize () ; counter . zeroize () ; flags . zeroize () ; } }
};
}
