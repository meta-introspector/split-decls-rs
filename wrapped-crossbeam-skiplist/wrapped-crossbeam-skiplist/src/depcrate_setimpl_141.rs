// Generated macro for impl_141 (impl)
macro_rules! Depcrate_setimpl_141 {
() => {
// Module: crate::set
// Provides: {"impl_141"}
// Dependencies: {}
impl < Q , R , T > fmt :: Debug for Range < '_ , Q , R , T > where T : Ord + Comparable < Q > + fmt :: Debug , R : RangeBounds < Q > + fmt :: Debug , Q : ? Sized , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Range") . field ("range" , & self . inner . inner . range) . field ("head" , & self . inner . inner . head . as_ref () . map (| e | e . key ())) . field ("tail" , & self . inner . inner . tail . as_ref () . map (| e | e . key ())) . finish () } }
};
}
