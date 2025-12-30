// Generated macro for impl_203 (impl)
macro_rules! Depcrate_dirwalk_iterimpl_203 {
() => {
// Module: crate::dirwalk::iter
// Provides: {"impl_203"}
// Dependencies: {}
# [cfg (feature = "parallel")] impl Drop for Iter { fn drop (& mut self) { crate :: util :: parallel_iter_drop (self . rx_and_join . take () . map (| (rx , handle) | (rx , handle , None :: < std :: thread :: JoinHandle < () > >)) , & self . should_interrupt ,) ; } }
};
}
