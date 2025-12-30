// Generated macro for impl_242 (impl)
macro_rules! Depcrateimpl_242 {
() => {
// Module: crate
// Provides: {"impl_242"}
// Dependencies: {}
impl Parse for ClassMarker { fn parse (input : ParseStream) -> SynResult < Self > { let class = input . parse :: < syn :: Ident > () ? ; input . parse :: < Token ! [=] > () ? ; let mut js_class = input . parse :: < syn :: LitStr > () ? . value () ; js_class = js_class . strip_prefix ("r#") . map (String :: from) . unwrap_or (js_class) ; let mut wasm_bindgen = None ; let mut wasm_bindgen_futures = None ; loop { if input . parse :: < Option < Token ! [,] > > () ? . is_some () { let ident = input . parse :: < syn :: Ident > () ? ; if ident == "wasm_bindgen" { if wasm_bindgen . is_some () { return Err (syn :: Error :: new (ident . span () , "found duplicate `wasm_bindgen`" ,)) ; } input . parse :: < Token ! [=] > () ? ; wasm_bindgen = Some (input . parse :: < syn :: Path > () ?) ; } else if ident == "wasm_bindgen_futures" { if wasm_bindgen_futures . is_some () { return Err (syn :: Error :: new (ident . span () , "found duplicate `wasm_bindgen_futures`" ,)) ; } input . parse :: < Token ! [=] > () ? ; wasm_bindgen_futures = Some (input . parse :: < syn :: Path > () ?) ; } else { return Err (syn :: Error :: new (ident . span () , "expected `wasm_bindgen` or `wasm_bindgen_futures`" ,)) ; } } else { break ; } } Ok (ClassMarker { class , js_class , wasm_bindgen : wasm_bindgen . unwrap_or_else (| | syn :: parse_quote ! { wasm_bindgen }) , wasm_bindgen_futures : wasm_bindgen_futures . unwrap_or_else (| | syn :: parse_quote ! { wasm_bindgen_futures }) , }) } }
};
}
