// Generated macro for ALL_VEC (const)
macro_rules! Depcrate_codepointinvlist_cpinvlistALL_VEC {
() => {
// Module: crate::codepointinvlist::cpinvlist
// Provides: {"ALL_VEC"}
// Dependencies: {}
# [doc = " Represents the inversion list for all of the code points in the Unicode range."] const ALL_VEC : ZeroVec < PotentialCodePoint > = zerovec ! (PotentialCodePoint ; PotentialCodePoint :: to_unaligned ; [PotentialCodePoint :: from_u24 (0x0) , PotentialCodePoint :: from_u24 ((char :: MAX as u32) + 1)]) ;
};
}
