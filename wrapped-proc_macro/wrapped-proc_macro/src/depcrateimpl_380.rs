// Generated macro for impl_380 (impl)
macro_rules! Depcrateimpl_380 {
() => {
// Module: crate
// Provides: {"impl_380"}
// Dependencies: {}
# [stable (feature = "proc_macro_lib2" , since = "1.29.0")] impl fmt :: Debug for Punct { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Punct") . field ("ch" , & self . as_char ()) . field ("spacing" , & self . spacing ()) . field ("span" , & self . span ()) . finish () } }
};
}
