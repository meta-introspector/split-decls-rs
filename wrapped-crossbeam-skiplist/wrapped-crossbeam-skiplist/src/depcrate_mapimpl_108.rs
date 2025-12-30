// Generated macro for impl_108 (impl)
macro_rules! Depcrate_mapimpl_108 {
() => {
// Module: crate::map
// Provides: {"impl_108"}
// Dependencies: {}
impl < Q , R , K , V > fmt :: Debug for Range < '_ , Q , R , K , V > where K : Ord + fmt :: Debug + Comparable < Q > , V : fmt :: Debug , R : RangeBounds < Q > + fmt :: Debug , Q : ? Sized , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Range") . field ("range" , & self . inner . range) . field ("head" , & self . inner . head) . field ("tail" , & self . inner . tail) . finish () } }
};
}
