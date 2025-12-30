// Generated macro for impl_62 (impl)
macro_rules! Depcrate_metadataimpl_62 {
() => {
// Module: crate::metadata
// Provides: {"impl_62"}
// Dependencies: {}
impl fmt :: Display for ClippyConfiguration { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "- `{}`: {}" , self . name , self . doc) ? ; if ! self . default . is_empty () { write ! (f , "\n\n   (default: `{}`)" , self . default) ? ; } Ok (()) } }
};
}
