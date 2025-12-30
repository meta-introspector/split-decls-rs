// Generated macro for impl_45 (impl)
macro_rules! Depcrateimpl_45 {
() => {
// Module: crate
// Provides: {"impl_45"}
// Dependencies: {}
impl < T > DoubleEndedIterator for IdxRange < T > { fn next_back (& mut self) -> Option < Self :: Item > { self . range . next_back () . map (| raw | Idx :: from_raw (raw . into ())) } }
};
}
