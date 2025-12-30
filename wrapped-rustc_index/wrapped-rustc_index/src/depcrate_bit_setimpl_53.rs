// Generated macro for impl_53 (impl)
macro_rules! Depcrate_bit_setimpl_53 {
() => {
// Module: crate::bit_set
// Provides: {"impl_53"}
// Dependencies: {}
impl < T > Clone for MixedBitSet < T > { fn clone (& self) -> Self { match self { MixedBitSet :: Small (set) => MixedBitSet :: Small (set . clone ()) , MixedBitSet :: Large (set) => MixedBitSet :: Large (set . clone ()) , } } # [doc = " WARNING: this implementation of clone_from may panic if the two"] # [doc = " bitsets have different domain sizes. This constraint is not inherent to"] # [doc = " `clone_from`, but it works with the existing call sites and allows a"] # [doc = " faster implementation, which is important because this function is hot."] fn clone_from (& mut self , from : & Self) { match (self , from) { (MixedBitSet :: Small (set) , MixedBitSet :: Small (from)) => set . clone_from (from) , (MixedBitSet :: Large (set) , MixedBitSet :: Large (from)) => set . clone_from (from) , _ => panic ! ("MixedBitSet size mismatch") , } } }
};
}
