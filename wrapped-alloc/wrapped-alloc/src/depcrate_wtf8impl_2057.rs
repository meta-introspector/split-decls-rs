// Generated macro for impl_2057 (impl)
macro_rules! Depcrate_wtf8impl_2057 {
() => {
// Module: crate::wtf8
// Provides: {"impl_2057"}
// Dependencies: {}
# [doc = " Formats the string with unpaired surrogates substituted with the replacement"] # [doc = " character, U+FFFD."] impl fmt :: Display for Wtf8Buf { fn fmt (& self , formatter : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if let Some (s) = self . as_known_utf8 () { fmt :: Display :: fmt (s , formatter) } else { fmt :: Display :: fmt (& * * self , formatter) } } }
};
}
