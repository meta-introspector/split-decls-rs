// Generated macro for impl_91 (impl)
macro_rules! Depcrate_noteimpl_91 {
() => {
// Module: crate::note
// Provides: {"impl_91"}
// Dependencies: {}
impl Preprocessor for TrplNote { fn name (& self) -> & str { "simple-note-preprocessor" } fn run (& self , _ctx : & PreprocessorContext , mut book : Book) -> Result < Book > { book . for_each_mut (| item | { if let BookItem :: Chapter (ref mut chapter) = item { chapter . content = rewrite (& chapter . content) ; } }) ; Ok (book) } fn supports_renderer (& self , renderer : & str) -> bool { renderer == "html" || renderer == "markdown" || renderer == "test" } }
};
}
