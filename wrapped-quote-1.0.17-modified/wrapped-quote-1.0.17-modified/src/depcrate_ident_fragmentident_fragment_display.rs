// Generated macro for ident_fragment_display (macro)
macro_rules! Depcrate_ident_fragmentident_fragment_display {
() => {
// Module: crate::ident_fragment
// Provides: {"ident_fragment_display"}
// Dependencies: {}
macro_rules ! ident_fragment_display { ($ ($ T : ty) ,*) => { $ (impl IdentFragment for $ T { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { fmt :: Display :: fmt (self , f) } }) * } ; }
};
}
