// Generated macro for impl_194 (impl)
macro_rules! Depcrate_parserimpl_194 {
() => {
// Module: crate::parser
// Provides: {"impl_194"}
// Dependencies: {}
impl < 'a > ConvertToAst < (& ast :: Program , BindgenAttrs , & 'a Option < ast :: ImportModule >) > for syn :: ForeignItemStatic { type Target = ast :: ImportKind ; fn convert (self , (program , opts , module) : (& ast :: Program , BindgenAttrs , & 'a Option < ast :: ImportModule >) ,) -> Result < Self :: Target , Diagnostic > { if let syn :: StaticMutability :: Mut (_) = self . mutability { bail_span ! (self . mutability , "cannot import mutable globals yet") } if let Some (span) = opts . static_string () { return Err (Diagnostic :: span_error (* span , "static strings require a string literal" ,)) ; } let default_name = self . ident . to_string () ; let js_name = opts . js_name () . map (| p | p . 0) . unwrap_or (& default_name) . to_string () ; let shim = format ! ("__wbg_static_accessor_{}_{}" , self . ident , ShortHash ((& js_name , module , & self . ident)) ,) ; let thread_local = opts . get_thread_local () ? ; opts . check_used () ; Ok (ast :: ImportKind :: Static (ast :: ImportStatic { ty : * self . ty , vis : self . vis , rust_name : self . ident . clone () , js_name , shim : Ident :: new (& shim , Span :: call_site ()) , wasm_bindgen : program . wasm_bindgen . clone () , thread_local , })) } }
};
}
