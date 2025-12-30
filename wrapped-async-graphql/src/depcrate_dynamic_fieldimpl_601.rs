// Generated macro for impl_601 (impl)
macro_rules! Depcrate_dynamic_fieldimpl_601 {
() => {
// Module: crate::dynamic::field
// Provides: {"impl_601"}
// Dependencies: {}
impl Debug for Field { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Field") . field ("name" , & self . name) . field ("description" , & self . description) . field ("arguments" , & self . arguments) . field ("ty" , & self . ty) . field ("deprecation" , & self . deprecation) . finish () } }
};
}
