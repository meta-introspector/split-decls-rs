// Generated macro for impl_52 (impl)
macro_rules! Depcrate_baseimpl_52 {
() => {
// Module: crate::base
// Provides: {"impl_52"}
// Dependencies: {}
impl < K , V > fmt :: Debug for Iter < '_ , '_ , K , V > where K : fmt :: Debug , V : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Iter") . field ("head" , & self . head . map (| n | (& n . key , & n . value))) . field ("tail" , & self . tail . map (| n | (& n . key , & n . value))) . finish () } }
};
}
