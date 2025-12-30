// Generated macro for impl_286 (impl)
macro_rules! Depcrate_stringimpl_286 {
() => {
// Module: crate::string
// Provides: {"impl_286"}
// Dependencies: {}
impl < 'a , LenT : LenType , const N : usize > iter :: FromIterator < & 'a str > for String < N , LenT > { fn from_iter < T : IntoIterator < Item = & 'a str > > (iter : T) -> Self { let mut new = Self :: new () ; for c in iter { new . push_str (c) . unwrap () ; } new } }
};
}
