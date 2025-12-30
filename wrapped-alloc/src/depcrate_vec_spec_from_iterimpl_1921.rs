// Generated macro for impl_1921 (impl)
macro_rules! Depcrate_vec_spec_from_iterimpl_1921 {
() => {
// Module: crate::vec::spec_from_iter
// Provides: {"impl_1921"}
// Dependencies: {}
impl < T > SpecFromIter < T , IntoIter < T > > for Vec < T > { # [track_caller] fn from_iter (iterator : IntoIter < T >) -> Self { let has_advanced = iterator . buf != iterator . ptr ; if ! has_advanced || iterator . len () >= iterator . cap / 2 { unsafe { let it = ManuallyDrop :: new (iterator) ; if has_advanced { ptr :: copy (it . ptr . as_ptr () , it . buf . as_ptr () , it . len ()) ; } return Vec :: from_parts (it . buf , it . len () , it . cap) ; } } let mut vec = Vec :: new () ; vec . spec_extend (iterator) ; vec } }
};
}
