// Generated macro for impl_17 (impl)
macro_rules! Depcrateimpl_17 {
() => {
// Module: crate
// Provides: {"impl_17"}
// Dependencies: {}
impl < B : BitBlock > FromIterator < usize > for BitSet < B > { fn from_iter < I : IntoIterator < Item = usize > > (iter : I) -> Self { let mut ret = Self :: default () ; ret . extend (iter) ; ret } }
};
}
