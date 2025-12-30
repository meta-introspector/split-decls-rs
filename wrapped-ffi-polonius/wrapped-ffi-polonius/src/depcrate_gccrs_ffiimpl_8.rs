// Generated macro for impl_8 (impl)
macro_rules! Depcrate_gccrs_ffiimpl_8 {
() => {
// Module: crate::gccrs_ffi
// Provides: {"impl_8"}
// Dependencies: {}
impl < OUT , IN > Into < Vec < OUT > > for Slice < IN > where IN : Into < OUT > + Copy , { fn into (self) -> Vec < OUT > { let slice = unsafe { std :: slice :: from_raw_parts (self . data , self . len as usize) } ; slice . iter () . map (| & e | e . into ()) . collect () } }
};
}
