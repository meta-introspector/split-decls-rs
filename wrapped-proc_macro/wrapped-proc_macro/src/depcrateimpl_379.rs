// Generated macro for impl_379 (impl)
macro_rules! Depcrateimpl_379 {
() => {
// Module: crate
// Provides: {"impl_379"}
// Dependencies: {}
# [doc = " Prints the punctuation character as a string that should be losslessly convertible"] # [doc = " back into the same character."] # [stable (feature = "proc_macro_lib2" , since = "1.29.0")] impl fmt :: Display for Punct { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{}" , self . as_char ()) } }
};
}
