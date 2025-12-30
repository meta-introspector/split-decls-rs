// Generated macro for impl_284 (impl)
macro_rules! Depcrate_stringimpl_284 {
() => {
// Module: crate::string
// Provides: {"impl_284"}
// Dependencies: {}
impl < LenT : LenType , const N : usize > iter :: FromIterator < char > for String < N , LenT > { fn from_iter < T : IntoIterator < Item = char > > (iter : T) -> Self { let mut new = Self :: new () ; for c in iter { new . push (c) . unwrap () ; } new } }
};
}
