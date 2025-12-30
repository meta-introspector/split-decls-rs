// Generated macro for impl_22 (impl)
macro_rules! Depcrate_globimpl_22 {
() => {
// Module: crate::glob
// Provides: {"impl_22"}
// Dependencies: {}
impl std :: fmt :: Debug for Glob { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { if f . alternate () { f . debug_struct ("Glob") . field ("glob" , & self . glob) . field ("re" , & self . re) . field ("opts" , & self . opts) . field ("tokens" , & self . tokens) . finish () } else { f . debug_tuple ("Glob") . field (& self . glob) . finish () } } }
};
}
