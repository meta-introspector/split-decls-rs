// Generated macro for impl_216 (impl)
macro_rules! Depcrate_errorimpl_216 {
() => {
// Module: crate::error
// Provides: {"impl_216"}
// Dependencies: {}
impl < 'a > fmt :: Display for Identifier < 'a > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if self . 0 . is_empty () || ! self . 0 . chars () . all (is_ident_raw_char) { return write ! (f , "{:?}_[invalid identifier]" , self . 0) ; } let mut chars = self . 0 . chars () ; if ! chars . next () . map_or (false , is_ident_first_char) || ! chars . all (is_xid_continue) { write ! (f , "`r#{}`" , self . 0) } else { write ! (f , "`{}`" , self . 0) } } }
};
}
