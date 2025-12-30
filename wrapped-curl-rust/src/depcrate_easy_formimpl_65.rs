// Generated macro for impl_65 (impl)
macro_rules! Depcrate_easy_formimpl_65 {
() => {
// Module: crate::easy::form
// Provides: {"impl_65"}
// Dependencies: {}
impl < 'form , 'data > fmt :: Debug for Part < 'form , 'data > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("Part") . field ("name" , & self . name) . field ("form" , & self . form) . finish () } }
};
}
