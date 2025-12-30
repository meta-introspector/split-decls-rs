// Generated macro for impl_317 (impl)
macro_rules! Depcrateimpl_317 {
() => {
// Module: crate
// Provides: {"impl_317"}
// Dependencies: {}
# [doc = " Prints token in a form convenient for debugging."] # [stable (feature = "proc_macro_lib" , since = "1.15.0")] impl fmt :: Debug for TokenStream { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str ("TokenStream ") ? ; f . debug_list () . entries (self . clone ()) . finish () } }
};
}
