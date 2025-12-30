// Generated macro for impl_626 (impl)
macro_rules! Depcrate_layoutimpl_626 {
() => {
// Module: crate::layout
// Provides: {"impl_626"}
// Dependencies: {}
impl rustc_index :: Idx for RustcFieldIdx { fn new (idx : usize) -> Self { RustcFieldIdx (Idx :: from_raw (RawIdx :: from (idx as u32))) } fn index (self) -> usize { u32 :: from (self . 0 . into_raw ()) as usize } }
};
}
