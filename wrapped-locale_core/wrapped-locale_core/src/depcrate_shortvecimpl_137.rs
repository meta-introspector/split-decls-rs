// Generated macro for impl_137 (impl)
macro_rules! Depcrate_shortvecimpl_137 {
() => {
// Module: crate::shortvec
// Provides: {"impl_137"}
// Dependencies: {}
impl < T > Deref for ShortBoxSlice < T > { type Target = [T] ; fn deref (& self) -> & Self :: Target { use ShortBoxSliceInner :: * ; match self . 0 { ZeroOne (None) => & [] , ZeroOne (Some (ref v)) => core :: slice :: from_ref (v) , # [cfg (feature = "alloc")] Multi (ref v) => v , # [cfg (not (feature = "alloc"))] Two (ref v) => v , } } }
};
}
