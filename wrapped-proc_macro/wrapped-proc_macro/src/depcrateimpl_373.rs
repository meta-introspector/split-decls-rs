// Generated macro for impl_373 (impl)
macro_rules! Depcrateimpl_373 {
() => {
// Module: crate
// Provides: {"impl_373"}
// Dependencies: {}
# [stable (feature = "proc_macro_lib2" , since = "1.29.0")] impl fmt :: Debug for Group { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Group") . field ("delimiter" , & self . delimiter ()) . field ("stream" , & self . stream ()) . field ("span" , & self . span ()) . finish () } }
};
}
