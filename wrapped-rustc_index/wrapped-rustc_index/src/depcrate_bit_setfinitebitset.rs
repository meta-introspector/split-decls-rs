// Generated macro for FiniteBitSet (struct)
macro_rules! Depcrate_bit_setFiniteBitSet {
() => {
// Module: crate::bit_set
// Provides: {"FiniteBitSet"}
// Dependencies: {}
# [doc = " A fixed-sized bitset type represented by an integer type. Indices outwith than the range"] # [doc = " representable by `T` are considered set."] # [cfg_attr (feature = "nightly" , derive (Decodable_NoContext , Encodable_NoContext))] # [derive (Copy , Clone , Eq , PartialEq)] pub struct FiniteBitSet < T : FiniteBitSetTy > (pub T) ;
};
}
