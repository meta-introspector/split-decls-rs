// Generated macro for impl_306 (impl)
macro_rules! Depcrate_parseimpl_306 {
() => {
// Module: crate::parse
// Provides: {"impl_306"}
// Dependencies: {}
impl VisitMut for CasesFunctionExtractor { fn visit_item_fn_mut (& mut self , node : & mut ItemFn) { let attrs = std :: mem :: take (& mut node . attrs) ; let mut attrs_buffer = Default :: default () ; let case : syn :: PathSegment = parse_quote ! { case } ; for attr in attrs . into_iter () { if attr_starts_with (& attr , & case) { match attr . parse_args :: < Expressions > () { Ok (expressions) => { let description = attr . path () . segments . iter () . nth (1) . map (| p | p . ident . clone ()) ; self . 0 . push (TestCase { args : expressions . into () , attrs : std :: mem :: take (& mut attrs_buffer) , description , }) ; } Err (err) => self . 1 . push (err) , } ; } else { attrs_buffer . push (attr) } } node . attrs = std :: mem :: take (& mut attrs_buffer) ; } }
};
}
