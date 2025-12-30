// Generated macro for impl_68 (impl)
macro_rules! Depcrate_apiimpl_68 {
() => {
// Module: crate::api
// Provides: {"impl_68"}
// Dependencies: {}
impl < L : Language > fmt :: Debug for SyntaxToken < L > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{:?}@{:?}" , self . kind () , self . text_range ()) ? ; if self . text () . len () < 25 { return write ! (f , " {:?}" , self . text ()) ; } let text = self . text () ; for idx in 21 .. 25 { if text . is_char_boundary (idx) { let text = format ! ("{} ..." , & text [.. idx]) ; return write ! (f , " {:?}" , text) ; } } unreachable ! () } }
};
}
