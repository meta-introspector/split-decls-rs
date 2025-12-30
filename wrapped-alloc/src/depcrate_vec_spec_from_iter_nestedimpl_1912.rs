// Generated macro for impl_1912 (impl)
macro_rules! Depcrate_vec_spec_from_iter_nestedimpl_1912 {
() => {
// Module: crate::vec::spec_from_iter_nested
// Provides: {"impl_1912"}
// Dependencies: {}
impl < T , I > SpecFromIterNested < T , I > for Vec < T > where I : Iterator < Item = T > , { # [track_caller] default fn from_iter (mut iterator : I) -> Self { let mut vector = match iterator . next () { None => return Vec :: new () , Some (element) => { let (lower , _) = iterator . size_hint () ; let initial_capacity = cmp :: max (RawVec :: < T > :: MIN_NON_ZERO_CAP , lower . saturating_add (1)) ; let mut vector = Vec :: with_capacity (initial_capacity) ; unsafe { ptr :: write (vector . as_mut_ptr () , element) ; vector . set_len (1) ; } vector } } ; < Vec < T > as SpecExtend < T , I > > :: spec_extend (& mut vector , iterator) ; vector } }
};
}
