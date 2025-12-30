// Generated macro for impl_890 (impl)
macro_rules! Depcrate_util_parse_attributeimpl_890 {
() => {
// Module: crate::util::parse_attribute
// Provides: {"impl_890"}
// Dependencies: {}
impl fmt :: Display for DisplayPath < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let path = self . 0 ; if path . leading_colon . is_some () { write ! (f , "::") ? ; } for segment in path . segments . pairs () { match segment { Pair :: Punctuated (segment , _) => write ! (f , "{}::" , segment . ident) ? , Pair :: End (segment) => segment . ident . fmt (f) ? , } } Ok (()) } }
};
}
