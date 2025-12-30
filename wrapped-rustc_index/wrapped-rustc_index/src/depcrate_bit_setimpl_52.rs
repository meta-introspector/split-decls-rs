// Generated macro for impl_52 (impl)
macro_rules! Depcrate_bit_setimpl_52 {
() => {
// Module: crate::bit_set
// Provides: {"impl_52"}
// Dependencies: {}
impl < T : Idx > MixedBitSet < T > { # [inline] pub fn new_empty (domain_size : usize) -> MixedBitSet < T > { if domain_size <= CHUNK_BITS { MixedBitSet :: Small (DenseBitSet :: new_empty (domain_size)) } else { MixedBitSet :: Large (ChunkedBitSet :: new_empty (domain_size)) } } # [inline] pub fn is_empty (& self) -> bool { match self { MixedBitSet :: Small (set) => set . is_empty () , MixedBitSet :: Large (set) => set . is_empty () , } } # [inline] pub fn contains (& self , elem : T) -> bool { match self { MixedBitSet :: Small (set) => set . contains (elem) , MixedBitSet :: Large (set) => set . contains (elem) , } } # [inline] pub fn insert (& mut self , elem : T) -> bool { match self { MixedBitSet :: Small (set) => set . insert (elem) , MixedBitSet :: Large (set) => set . insert (elem) , } } pub fn insert_all (& mut self) { match self { MixedBitSet :: Small (set) => set . insert_all () , MixedBitSet :: Large (set) => set . insert_all () , } } # [inline] pub fn remove (& mut self , elem : T) -> bool { match self { MixedBitSet :: Small (set) => set . remove (elem) , MixedBitSet :: Large (set) => set . remove (elem) , } } pub fn iter (& self) -> MixedBitIter < '_ , T > { match self { MixedBitSet :: Small (set) => MixedBitIter :: Small (set . iter ()) , MixedBitSet :: Large (set) => MixedBitIter :: Large (set . iter ()) , } } # [inline] pub fn clear (& mut self) { match self { MixedBitSet :: Small (set) => set . clear () , MixedBitSet :: Large (set) => set . clear () , } } bit_relations_inherent_impls ! { } }
};
}
