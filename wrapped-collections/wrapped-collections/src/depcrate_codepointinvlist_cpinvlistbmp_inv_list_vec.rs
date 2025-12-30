// Generated macro for BMP_INV_LIST_VEC (const)
macro_rules! Depcrate_codepointinvlist_cpinvlistBMP_INV_LIST_VEC {
() => {
// Module: crate::codepointinvlist::cpinvlist
// Provides: {"BMP_INV_LIST_VEC"}
// Dependencies: {}
# [doc = " Represents the inversion list for a set of all code points in the Basic Multilingual Plane."] const BMP_INV_LIST_VEC : ZeroVec < PotentialCodePoint > = zerovec ! (PotentialCodePoint ; PotentialCodePoint :: to_unaligned ; [PotentialCodePoint :: from_u24 (0x0) , PotentialCodePoint :: from_u24 (BMP_MAX + 1)]) ;
};
}
