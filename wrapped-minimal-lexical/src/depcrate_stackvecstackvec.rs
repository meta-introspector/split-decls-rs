// Generated macro for StackVec (struct)
macro_rules! Depcrate_stackvecStackVec {
() => {
// Module: crate::stackvec
// Provides: {"StackVec"}
// Dependencies: {}
# [doc = " Simple stack vector implementation."] # [derive (Clone)] pub struct StackVec { # [doc = " The raw buffer for the elements."] data : [mem :: MaybeUninit < bigint :: Limb > ; bigint :: BIGINT_LIMBS] , # [doc = " The number of elements in the array (we never need more than u16::MAX)."] length : u16 , }
};
}
