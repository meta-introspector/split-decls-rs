// Generated macro for impl_130 (impl)
macro_rules! Depcrateimpl_130 {
() => {
// Module: crate
// Provides: {"impl_130"}
// Dependencies: {}
impl < T : Num + Clone > Sum for Complex < T > { fn sum < I > (iter : I) -> Self where I : Iterator < Item = Self > , { iter . fold (Self :: zero () , | acc , c | acc + c) } }
};
}
