// Generated macro for impl_78 (impl)
macro_rules! Depcrate_prettyimpl_78 {
() => {
// Module: crate::pretty
// Provides: {"impl_78"}
// Dependencies: {}
impl < 'a > pprust_ast :: PpAnn for AstHygieneAnn < 'a > { fn post (& self , s : & mut pprust_ast :: State < '_ > , node : pprust_ast :: AnnNode < '_ >) { match node { pprust_ast :: AnnNode :: Ident (& Ident { name , span }) => { s . s . space () ; s . synth_comment (format ! ("{}{:?}" , name . as_u32 () , span . ctxt ())) } pprust_ast :: AnnNode :: Name (& name) => { s . s . space () ; s . synth_comment (name . as_u32 () . to_string ()) } pprust_ast :: AnnNode :: Crate (_) => { s . s . hardbreak () ; let verbose = self . sess . verbose_internals () ; s . synth_comment (rustc_span :: hygiene :: debug_hygiene_data (verbose)) ; s . s . hardbreak_if_not_bol () ; } _ => { } } } }
};
}
