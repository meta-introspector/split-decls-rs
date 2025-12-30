// Generated macro for impl_285 (impl)
macro_rules! Depcrate_stringimpl_285 {
() => {
// Module: crate::string
// Provides: {"impl_285"}
// Dependencies: {}
impl < 'a , LenT : LenType , const N : usize > iter :: FromIterator < & 'a char > for String < N , LenT > { fn from_iter < T : IntoIterator < Item = & 'a char > > (iter : T) -> Self { let mut new = Self :: new () ; for c in iter { new . push (* c) . unwrap () ; } new } }
};
}
