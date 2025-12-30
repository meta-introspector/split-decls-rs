// Generated macro for impl_67 (impl)
macro_rules! Depcrate_listingimpl_67 {
() => {
// Module: crate::listing
// Provides: {"impl_67"}
// Dependencies: {}
impl Preprocessor for TrplListing { fn name (& self) -> & str { "trpl-listing" } fn run (& self , ctx : & PreprocessorContext , mut book : Book) -> Result < Book > { let mode = Mode :: from_context (ctx , self . name ()) ? ; let mut errors = vec ! [] ; book . for_each_mut (| item | { if let BookItem :: Chapter (ref mut chapter) = item { match rewrite_listing (& chapter . content , mode) { Ok (rewritten) => chapter . content = rewritten , Err (reason) => errors . push (anyhow ! (reason)) , } } }) ; if errors . is_empty () { Ok (book) } else { Err (CompositeError (errors) . into ()) } } fn supports_renderer (& self , renderer : & str) -> bool { renderer == "html" || renderer == "markdown" || renderer == "test" } }
};
}
