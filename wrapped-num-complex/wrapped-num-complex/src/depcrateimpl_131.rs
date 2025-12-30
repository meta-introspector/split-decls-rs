// Generated macro for impl_131 (impl)
macro_rules! Depcrateimpl_131 {
() => {
// Module: crate
// Provides: {"impl_131"}
// Dependencies: {}
impl < 'a , T : 'a + Num + Clone > Sum < & 'a Complex < T > > for Complex < T > { fn sum < I > (iter : I) -> Self where I : Iterator < Item = & 'a Complex < T > > , { iter . fold (Self :: zero () , | acc , c | acc + c) } }
};
}
