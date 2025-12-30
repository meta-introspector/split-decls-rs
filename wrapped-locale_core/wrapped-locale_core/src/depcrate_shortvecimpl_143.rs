// Generated macro for impl_143 (impl)
macro_rules! Depcrate_shortvecimpl_143 {
() => {
// Module: crate::shortvec
// Provides: {"impl_143"}
// Dependencies: {}
impl < T > Iterator for ShortBoxSliceIntoIter < T > { type Item = T ; fn next (& mut self) -> Option < T > { use ShortBoxSliceIntoIterInner :: * ; match & mut self . 0 { ZeroOne (option) => option . take () , # [cfg (feature = "alloc")] Multi (into_iter) => into_iter . next () , # [cfg (not (feature = "alloc"))] Two (into_iter) => into_iter . next () , } } }
};
}
