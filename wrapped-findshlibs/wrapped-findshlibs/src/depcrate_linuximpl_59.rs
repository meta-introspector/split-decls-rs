// Generated macro for impl_59 (impl)
macro_rules! Depcrate_linuximpl_59 {
() => {
// Module: crate::linux
// Provides: {"impl_59"}
// Dependencies: {}
impl < 'a > fmt :: Debug for SegmentIter < 'a > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let ref phdr = self . inner . as_slice () [0] ; f . debug_struct ("SegmentIter") . field ("phdr" , & DebugPhdr (phdr)) . finish () } }
};
}
