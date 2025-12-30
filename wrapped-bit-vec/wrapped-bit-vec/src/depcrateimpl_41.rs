// Generated macro for impl_41 (impl)
macro_rules! Depcrateimpl_41 {
() => {
// Module: crate
// Provides: {"impl_41"}
// Dependencies: {}
impl < B : BitBlock > FromIterator < bool > for BitVec < B > { # [inline] fn from_iter < I : IntoIterator < Item = bool > > (iter : I) -> Self { let mut ret : Self = Default :: default () ; ret . extend (iter) ; ret } }
};
}
