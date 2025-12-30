// Generated macro for impl_41 (impl)
macro_rules! Depcrate_inputimpl_41 {
() => {
// Module: crate::input
// Provides: {"impl_41"}
// Dependencies: {}
impl fmt :: Debug for CrateGraphBuilder { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_map () . entries (self . arena . iter () . map (| (id , data) | (u32 :: from (id . into_raw ()) , data))) . finish () } }
};
}
