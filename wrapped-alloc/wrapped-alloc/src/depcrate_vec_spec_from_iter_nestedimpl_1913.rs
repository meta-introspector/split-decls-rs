// Generated macro for impl_1913 (impl)
macro_rules! Depcrate_vec_spec_from_iter_nestedimpl_1913 {
() => {
// Module: crate::vec::spec_from_iter_nested
// Provides: {"impl_1913"}
// Dependencies: {}
impl < T , I > SpecFromIterNested < T , I > for Vec < T > where I : TrustedLen < Item = T > , { # [track_caller] fn from_iter (iterator : I) -> Self { let mut vector = match iterator . size_hint () { (_ , Some (upper)) => Vec :: with_capacity (upper) , _ => panic ! ("capacity overflow") , } ; vector . spec_extend (iterator) ; vector } }
};
}
