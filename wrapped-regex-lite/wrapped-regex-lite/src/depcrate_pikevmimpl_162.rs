// Generated macro for impl_162 (impl)
macro_rules! Depcrate_pikevmimpl_162 {
() => {
// Module: crate::pikevm
// Provides: {"impl_162"}
// Dependencies: {}
impl core :: fmt :: Debug for SparseSet { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { let elements : Vec < StateID > = self . iter () . collect () ; f . debug_tuple ("SparseSet") . field (& elements) . finish () } }
};
}
