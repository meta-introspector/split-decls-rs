// Generated macro for impl_139 (impl)
macro_rules! Depcrate_shortvecimpl_139 {
() => {
// Module: crate::shortvec
// Provides: {"impl_139"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < T > From < Vec < T > > for ShortBoxSlice < T > { fn from (v : Vec < T >) -> Self { use ShortBoxSliceInner :: * ; match v . len () { 0 => Self (ZeroOne (None)) , # [expect (clippy :: unwrap_used)] 1 => Self (ZeroOne (Some (v . into_iter () . next () . unwrap ()))) , _ => Self (Multi (v . into_boxed_slice ())) , } } }
};
}
