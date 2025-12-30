// Generated macro for impl_282 (impl)
macro_rules! Depcrateimpl_282 {
() => {
// Module: crate
// Provides: {"impl_282"}
// Dependencies: {}
# [doc = " Prints the punctuation character as a string that should be losslessly"] # [doc = " convertible back into the same character."] impl Display for Punct { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { Display :: fmt (& self . ch , f) } }
};
}
