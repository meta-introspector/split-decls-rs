// Generated macro for impl_138 (impl)
macro_rules! Depcrate_shortvecimpl_138 {
() => {
// Module: crate::shortvec
// Provides: {"impl_138"}
// Dependencies: {}
impl < T > DerefMut for ShortBoxSlice < T > { fn deref_mut (& mut self) -> & mut Self :: Target { use ShortBoxSliceInner :: * ; match self . 0 { ZeroOne (None) => & mut [] , ZeroOne (Some (ref mut v)) => core :: slice :: from_mut (v) , # [cfg (feature = "alloc")] Multi (ref mut v) => v , # [cfg (not (feature = "alloc"))] Two (ref mut v) => v , } } }
};
}
