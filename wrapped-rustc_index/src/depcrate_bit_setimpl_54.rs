// Generated macro for impl_54 (impl)
macro_rules! Depcrate_bit_setimpl_54 {
() => {
// Module: crate::bit_set
// Provides: {"impl_54"}
// Dependencies: {}
impl < T : Idx > BitRelations < MixedBitSet < T > > for MixedBitSet < T > { fn union (& mut self , other : & MixedBitSet < T >) -> bool { match (self , other) { (MixedBitSet :: Small (set) , MixedBitSet :: Small (other)) => set . union (other) , (MixedBitSet :: Large (set) , MixedBitSet :: Large (other)) => set . union (other) , _ => panic ! ("MixedBitSet size mismatch") , } } fn subtract (& mut self , other : & MixedBitSet < T >) -> bool { match (self , other) { (MixedBitSet :: Small (set) , MixedBitSet :: Small (other)) => set . subtract (other) , (MixedBitSet :: Large (set) , MixedBitSet :: Large (other)) => set . subtract (other) , _ => panic ! ("MixedBitSet size mismatch") , } } fn intersect (& mut self , _other : & MixedBitSet < T >) -> bool { unimplemented ! ("implement if/when necessary") ; } }
};
}
