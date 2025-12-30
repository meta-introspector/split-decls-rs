// Generated macro for impl_662 (impl)
macro_rules! Depcrate_dynamic_scalarimpl_662 {
() => {
// Module: crate::dynamic::scalar
// Provides: {"impl_662"}
// Dependencies: {}
impl Debug for Scalar { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Scalar") . field ("name" , & self . name) . field ("description" , & self . description) . field ("specified_by_url" , & self . specified_by_url) . field ("inaccessible" , & self . inaccessible) . field ("tags" , & self . tags) . field ("requires_scopes" , & self . requires_scopes) . finish () } }
};
}
