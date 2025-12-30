// Generated macro for impl_6 (impl)
macro_rules! Depcrate_gccrs_ffiimpl_6 {
() => {
// Module: crate::gccrs_ffi
// Provides: {"impl_6"}
// Dependencies: {}
impl < T1 , T2 > Into < (GccrsAtom , GccrsAtom) > for Pair < T1 , T2 > where GccrsAtom : From < T1 > + From < T2 > , { fn into (self) -> (GccrsAtom , GccrsAtom) { (self . first . into () , self . second . into ()) } }
};
}
