// Generated macro for impl_125 (impl)
macro_rules! Depcrate_arrayvecimpl_125 {
() => {
// Module: crate::arrayvec
// Provides: {"impl_125"}
// Dependencies: {}
# [cfg (feature = "borsh")] # [doc = " Requires crate feature `\"borsh\"`"] impl < T , const CAP : usize > borsh :: BorshSerialize for ArrayVec < T , CAP > where T : borsh :: BorshSerialize , { fn serialize < W : borsh :: io :: Write > (& self , writer : & mut W) -> borsh :: io :: Result < () > { < [T] as borsh :: BorshSerialize > :: serialize (self . as_slice () , writer) } }
};
}
