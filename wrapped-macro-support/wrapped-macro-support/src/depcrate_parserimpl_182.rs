// Generated macro for impl_182 (impl)
macro_rules! Depcrate_parserimpl_182 {
() => {
// Module: crate::parser
// Provides: {"impl_182"}
// Dependencies: {}
impl Parse for BindgenAttrs { fn parse (input : ParseStream) -> SynResult < Self > { let mut attrs = BindgenAttrs :: default () ; if input . is_empty () { return Ok (attrs) ; } let opts = syn :: punctuated :: Punctuated :: < _ , syn :: token :: Comma > :: parse_terminated (input) ? ; attrs . attrs = opts . into_iter () . map (| c | (Cell :: new (false) , c)) . collect () ; Ok (attrs) } }
};
}
