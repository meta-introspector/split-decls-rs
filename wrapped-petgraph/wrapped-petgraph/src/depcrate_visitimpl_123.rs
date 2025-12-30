// Generated macro for impl_123 (impl)
macro_rules! Depcrate_visitimpl_123 {
() => {
// Module: crate::visit
// Provides: {"impl_123"}
// Dependencies: {}
impl < Ix > VisitMap < Ix > for FixedBitSet where Ix : IndexType , { fn visit (& mut self , x : Ix) -> bool { ! self . put (x . index ()) } fn is_visited (& self , x : & Ix) -> bool { self . contains (x . index ()) } fn unvisit (& mut self , x : Ix) -> bool { if self . is_visited (& x) { self . toggle (x . index ()) ; return true ; } false } }
};
}
