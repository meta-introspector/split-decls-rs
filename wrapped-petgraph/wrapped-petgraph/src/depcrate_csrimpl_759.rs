// Generated macro for impl_759 (impl)
macro_rules! Depcrate_csrimpl_759 {
() => {
// Module: crate::csr
// Provides: {"impl_759"}
// Dependencies: {}
impl < N , E , Ty , Ix > Visitable for Csr < N , E , Ty , Ix > where Ty : EdgeType , Ix : IndexType , { type Map = FixedBitSet ; fn visit_map (& self) -> FixedBitSet { FixedBitSet :: with_capacity (self . node_count ()) } fn reset_map (& self , map : & mut Self :: Map) { map . clear () ; map . grow (self . node_count ()) ; } }
};
}
