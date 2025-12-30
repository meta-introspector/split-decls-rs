// Generated macro for impl_27 (impl)
macro_rules! Depcrate_figureimpl_27 {
() => {
// Module: crate::figure
// Provides: {"impl_27"}
// Dependencies: {}
impl Preprocessor for TrplFigure { fn name (& self) -> & str { "trpl-figure" } fn run (& self , ctx : & mdbook :: preprocess :: PreprocessorContext , mut book : Book ,) -> Result < Book > { let Mode :: Simple = Mode :: from_context (ctx , self . name ()) ? else { return Ok (book) ; } ; let mut errors = vec ! [] ; book . for_each_mut (| item | { if let BookItem :: Chapter (ref mut chapter) = item { match rewrite_figure (& chapter . content) { Ok (rewritten) => chapter . content = rewritten , Err (reason) => errors . push (reason) , } } }) ; if errors . is_empty () { Ok (book) } else { Err (CompositeError (errors) . into ()) } } }
};
}
