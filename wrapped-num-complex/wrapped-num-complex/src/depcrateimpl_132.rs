// Generated macro for impl_132 (impl)
macro_rules! Depcrateimpl_132 {
() => {
// Module: crate
// Provides: {"impl_132"}
// Dependencies: {}
impl < T : Num + Clone > Product for Complex < T > { fn product < I > (iter : I) -> Self where I : Iterator < Item = Self > , { iter . fold (Self :: one () , | acc , c | acc * c) } }
};
}
