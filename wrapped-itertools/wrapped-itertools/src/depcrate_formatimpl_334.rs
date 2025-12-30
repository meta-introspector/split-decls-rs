// Generated macro for impl_334 (impl)
macro_rules! Depcrate_formatimpl_334 {
() => {
// Module: crate::format
// Provides: {"impl_334"}
// Dependencies: {}
impl < I , F > fmt :: Debug for FormatWith < '_ , I , F > where I : Iterator , F : FnMut (I :: Item , & mut dyn FnMut (& dyn fmt :: Display) -> fmt :: Result) -> fmt :: Result , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Display :: fmt (self , f) } }
};
}
