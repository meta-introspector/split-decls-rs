// Generated macro for impl_7965 (impl)
macro_rules! Depcrate_no_mangle_with_rust_abiimpl_7965 {
() => {
// Module: crate::no_mangle_with_rust_abi
// Provides: {"impl_7965"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for NoMangleWithRustAbi { fn check_item (& mut self , cx : & LateContext < 'tcx > , item : & 'tcx Item < 'tcx >) { if let ItemKind :: Fn { ident , sig : fn_sig , .. } = & item . kind && ! item . span . from_expansion () { let attrs = cx . tcx . hir_attrs (item . hir_id ()) ; let mut app = Applicability :: MaybeIncorrect ; let fn_snippet = snippet_with_applicability (cx , fn_sig . span . with_hi (ident . span . lo ()) , ".." , & mut app) ; for attr in attrs { if let Attribute :: Parsed (AttributeKind :: NoMangle (attr_span)) = attr && fn_sig . header . abi == ExternAbi :: Rust && let Some ((fn_attrs , _)) = fn_snippet . rsplit_once ("fn") && ! fn_attrs . contains ("extern") { let sugg_span = fn_sig . span . with_lo (fn_sig . span . lo () + BytePos :: from_usize (fn_attrs . len ())) . shrink_to_lo () ; let attr_snippet = snippet (cx , * attr_span , "..") ; span_lint_and_then (cx , NO_MANGLE_WITH_RUST_ABI , fn_sig . span , format ! ("`{attr_snippet}` set on a function with the default (`Rust`) ABI") , | diag | { diag . span_suggestion (sugg_span , "set an ABI" , "extern \"C\" " , app) . span_suggestion (sugg_span , "or explicitly set the default" , "extern \"Rust\" " , app) ; } ,) ; } } } } }
};
}
