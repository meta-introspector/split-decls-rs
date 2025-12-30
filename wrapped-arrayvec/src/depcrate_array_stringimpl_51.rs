// Generated macro for impl_51 (impl)
macro_rules! Depcrate_array_stringimpl_51 {
() => {
// Module: crate::array_string
// Provides: {"impl_51"}
// Dependencies: {}
# [cfg (feature = "borsh")] # [doc = " Requires crate feature `\"borsh\"`"] impl < const CAP : usize > borsh :: BorshSerialize for ArrayString < CAP > { fn serialize < W : borsh :: io :: Write > (& self , writer : & mut W) -> borsh :: io :: Result < () > { < str as borsh :: BorshSerialize > :: serialize (& * self , writer) } }
};
}
