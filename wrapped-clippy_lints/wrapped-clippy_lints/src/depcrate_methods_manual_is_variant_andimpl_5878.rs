// Generated macro for impl_5878 (impl)
macro_rules! Depcrate_methods_manual_is_variant_andimpl_5878 {
() => {
// Module: crate::methods::manual_is_variant_and
// Provides: {"impl_5878"}
// Dependencies: {}
impl < 'hir > MapFunc < 'hir > { # [doc = " Build a suggestion suitable for use in a `.map()`-like function. η-expansion will be applied"] # [doc = " as needed."] fn sugg (self , cx : & LateContext < 'hir > , invert : bool , app : & mut Applicability) -> String { match self { Self :: Closure (closure) => { let body = Sugg :: hir_with_applicability (cx , cx . tcx . hir_body (closure . body) . value , ".." , app) ; format ! ("{} {}" , snippet_with_applicability (cx , closure . fn_decl_span , "|..|" , app) , if invert { ! body } else { body }) } , Self :: Path (expr) => { let path = snippet_with_applicability (cx , expr . span , "_" , app) ; if invert { format ! ("|x| !{path}(x)") } else { path . to_string () } } , } } }
};
}
