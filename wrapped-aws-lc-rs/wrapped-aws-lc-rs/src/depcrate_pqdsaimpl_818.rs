// Generated macro for impl_818 (impl)
macro_rules! Depcrate_pqdsaimpl_818 {
() => {
// Module: crate::pqdsa
// Provides: {"impl_818"}
// Dependencies: {}
impl AlgorithmID { # [allow (dead_code)] pub (crate) const fn from_nid (nid : c_int) -> Result < Self , Unspecified > { match nid { NID_MLDSA44 => Ok (Self :: ML_DSA_44) , NID_MLDSA65 => Ok (Self :: ML_DSA_65) , NID_MLDSA87 => Ok (Self :: ML_DSA_87) , _ => Err (Unspecified) , } } pub (crate) const fn nid (& self) -> c_int { match self { Self :: ML_DSA_44 => NID_MLDSA44 , Self :: ML_DSA_65 => NID_MLDSA65 , Self :: ML_DSA_87 => NID_MLDSA87 , } } # [allow (dead_code)] pub (crate) const fn priv_key_size_bytes (& self) -> usize { match self { Self :: ML_DSA_44 => 2560 , Self :: ML_DSA_65 => 4032 , Self :: ML_DSA_87 => 4896 , } } pub (crate) const fn pub_key_size_bytes (& self) -> usize { match self { Self :: ML_DSA_44 => 1312 , Self :: ML_DSA_65 => 1952 , Self :: ML_DSA_87 => 2592 , } } pub (crate) const fn signature_size_bytes (& self) -> usize { match self { Self :: ML_DSA_44 => 2420 , Self :: ML_DSA_65 => 3309 , Self :: ML_DSA_87 => 4627 , } } }
};
}
