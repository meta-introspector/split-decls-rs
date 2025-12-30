// Generated macro for impl_64 (impl)
macro_rules! Depcrate_baseimpl_64 {
() => {
// Module: crate::base
// Provides: {"impl_64"}
// Dependencies: {}
impl < Q , R , K , V > fmt :: Debug for RefRange < '_ , Q , R , K , V > where K : Ord + fmt :: Debug + Comparable < Q > , V : fmt :: Debug , R : RangeBounds < Q > + fmt :: Debug , Q : ? Sized , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("RefRange") . field ("range" , & self . range) . field ("head" , & self . head) . field ("tail" , & self . tail) . finish () } }
};
}
