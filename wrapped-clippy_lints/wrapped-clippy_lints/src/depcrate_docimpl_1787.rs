// Generated macro for impl_1787 (impl)
macro_rules! Depcrate_docimpl_1787 {
() => {
// Module: crate::doc
// Provides: {"impl_1787"}
// Dependencies: {}
impl Fragments < '_ > { # [doc = " get the span for the markdown range. Note that this function is not cheap, use it with"] # [doc = " caution."] # [must_use] fn span (self , cx : & LateContext < '_ > , range : Range < usize >) -> Option < Span > { source_span_for_markdown_range (cx . tcx , self . doc , & range , self . fragments) . map (| (sp , _) | sp) } }
};
}
