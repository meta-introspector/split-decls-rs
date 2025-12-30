// Generated macro for impl_386 (impl)
macro_rules! Depcrateimpl_386 {
() => {
// Module: crate
// Provides: {"impl_386"}
// Dependencies: {}
# [stable (feature = "proc_macro_lib2" , since = "1.29.0")] impl fmt :: Debug for Ident { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Ident") . field ("ident" , & self . to_string ()) . field ("span" , & self . span ()) . finish () } }
};
}
