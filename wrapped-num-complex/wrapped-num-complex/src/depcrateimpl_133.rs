// Generated macro for impl_133 (impl)
macro_rules! Depcrateimpl_133 {
() => {
// Module: crate
// Provides: {"impl_133"}
// Dependencies: {}
impl < 'a , T : 'a + Num + Clone > Product < & 'a Complex < T > > for Complex < T > { fn product < I > (iter : I) -> Self where I : Iterator < Item = & 'a Complex < T > > , { iter . fold (Self :: one () , | acc , c | acc * c) } }
};
}
