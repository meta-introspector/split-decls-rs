// Generated macro for impl_83 (impl)
macro_rules! Depcrate_codepointinvlist_cpinvlistimpl_83 {
() => {
// Module: crate::codepointinvlist::cpinvlist
// Provides: {"impl_83"}
// Dependencies: {}
# [cfg (feature = "serde")] impl core :: fmt :: Display for UnicodeCodePoint { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { match self . 0 { s @ 0xD800 ..= 0xDFFF => write ! (f , "U+{s:X}") , c => write ! (f , "{}" , char :: from_u32 (c) . unwrap_or (char :: REPLACEMENT_CHARACTER)) , } } }
};
}
