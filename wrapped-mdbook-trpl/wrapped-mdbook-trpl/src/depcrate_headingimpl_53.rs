// Generated macro for impl_53 (impl)
macro_rules! Depcrate_headingimpl_53 {
() => {
// Module: crate::heading
// Provides: {"impl_53"}
// Dependencies: {}
impl Preprocessor for TrplHeading { fn name (& self) -> & str { "trpl-heading" } fn run (& self , ctx : & PreprocessorContext , mut book : Book ,) -> anyhow :: Result < Book > { let mode = Mode :: from_context (ctx , self . name ()) ? ; let mut errors = vec ! [] ; book . for_each_mut (| item | { if let BookItem :: Chapter (ref mut chapter) = item { match rewrite_headings (& chapter . content , mode) { Ok (rewritten) => chapter . content = rewritten , Err (reason) => errors . push (reason) , } } }) ; if errors . is_empty () { Ok (book) } else { Err (CompositeError (errors) . into ()) } } fn supports_renderer (& self , renderer : & str) -> bool { renderer == "html" || renderer == "markdown" || renderer == "test" } }
};
}
