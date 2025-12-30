// Generated macro for impl_60 (impl)
macro_rules! Depcrate_baseimpl_60 {
() => {
// Module: crate::base
// Provides: {"impl_60"}
// Dependencies: {}
impl < Q , R , K , V > fmt :: Debug for Range < '_ , '_ , Q , R , K , V > where K : Ord + fmt :: Debug + Comparable < Q > , V : fmt :: Debug , R : RangeBounds < Q > + fmt :: Debug , Q : ? Sized , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Range") . field ("range" , & self . range) . field ("head" , & self . head) . field ("tail" , & self . tail) . finish () } }
};
}
