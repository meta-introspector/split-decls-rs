// Generated macro for prepare_for_impl_recursion (function)
macro_rules! Depcrate_parserprepare_for_impl_recursion {
() => {
// Module: crate::parser
// Provides: {"prepare_for_impl_recursion"}
// Dependencies: {}
fn prepare_for_impl_recursion (item : & mut syn :: ImplItem , class : & syn :: Path , program : & ast :: Program , impl_opts : & BindgenAttrs ,) -> Result < () , Diagnostic > { let method = match item { syn :: ImplItem :: Fn (m) => m , syn :: ImplItem :: Const (_) => { bail_span ! (&* item , "const definitions aren't supported with #[wasm_bindgen]") ; } syn :: ImplItem :: Type (_) => bail_span ! (&* item , "type definitions in impls aren't supported with #[wasm_bindgen]") , syn :: ImplItem :: Macro (_) => { bail_span ! (&* item , "macros in impls aren't supported") ; } syn :: ImplItem :: Verbatim (_) => panic ! ("unparsed impl item?") , other => bail_span ! (other , "failed to parse this item as a known item") , } ; let ident = extract_path_ident (class) ? ; let js_class = impl_opts . js_class () . map (| s | s . 0 . to_string ()) . unwrap_or (ident . to_string ()) ; let wasm_bindgen = & program . wasm_bindgen ; let wasm_bindgen_futures = & program . wasm_bindgen_futures ; method . attrs . insert (0 , syn :: Attribute { pound_token : Default :: default () , style : syn :: AttrStyle :: Outer , bracket_token : Default :: default () , meta : syn :: parse_quote ! { # wasm_bindgen :: prelude :: __wasm_bindgen_class_marker (# class = # js_class , wasm_bindgen = # wasm_bindgen , wasm_bindgen_futures = # wasm_bindgen_futures) } , } ,) ; Ok (()) }
};
}
