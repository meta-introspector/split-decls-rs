// Generated macro for impl_351 (impl)
macro_rules! Depcrate_adjimpl_351 {
() => {
// Module: crate::adj
// Provides: {"impl_351"}
// Dependencies: {}
impl < E , Ix > visit :: Visitable for List < E , Ix > where Ix : IndexType , { type Map = FixedBitSet ; fn visit_map (& self) -> FixedBitSet { FixedBitSet :: with_capacity (self . node_count ()) } fn reset_map (& self , map : & mut Self :: Map) { map . clear () ; map . grow (self . node_count ()) ; } }
};
}
