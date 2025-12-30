// Generated macro for impl_385 (impl)
macro_rules! Depcrateimpl_385 {
() => {
// Module: crate
// Provides: {"impl_385"}
// Dependencies: {}
# [doc = " Prints the identifier as a string that should be losslessly convertible back"] # [doc = " into the same identifier."] # [stable (feature = "proc_macro_lib2" , since = "1.29.0")] impl fmt :: Display for Ident { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if self . 0 . is_raw { f . write_str ("r#") ? ; } fmt :: Display :: fmt (& self . 0 . sym , f) } }
};
}
