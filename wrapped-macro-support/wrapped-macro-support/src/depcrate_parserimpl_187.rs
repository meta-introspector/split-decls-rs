// Generated macro for impl_187 (impl)
macro_rules! Depcrate_parserimpl_187 {
() => {
// Module: crate::parser
// Provides: {"impl_187"}
// Dependencies: {}
impl Parse for AnyIdent { fn parse (input : ParseStream) -> SynResult < Self > { input . step (| cursor | match cursor . ident () { Some ((ident , remaining)) => Ok ((AnyIdent (ident) , remaining)) , None => Err (cursor . error ("expected an identifier")) , }) } }
};
}
