// Generated macro for impl_7 (impl)
macro_rules! Depcrate_gccrs_ffiimpl_7 {
() => {
// Module: crate::gccrs_ffi
// Provides: {"impl_7"}
// Dependencies: {}
impl < T1 , T2 , T3 > Into < (GccrsAtom , GccrsAtom , GccrsAtom) > for Triple < T1 , T2 , T3 > where GccrsAtom : From < T1 > + From < T2 > + From < T3 > , { fn into (self) -> (GccrsAtom , GccrsAtom , GccrsAtom) { (self . first . into () , self . second . into () , self . third . into ()) } }
};
}
