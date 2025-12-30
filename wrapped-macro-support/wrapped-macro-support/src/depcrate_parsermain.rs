// Generated macro for main (function)
macro_rules! Depcrate_parsermain {
() => {
// Module: crate::parser
// Provides: {"main"}
// Dependencies: {}
fn main (program : & ast :: Program , mut f : ItemFn , tokens : & mut TokenStream) -> Result < () , Diagnostic > { if f . sig . ident != "main" { bail_span ! (& f . sig . ident , "the main function has to be called main") ; } if let Some (constness) = f . sig . constness { bail_span ! (& constness , "the main function cannot be const") ; } if ! f . sig . generics . params . is_empty () { bail_span ! (& f . sig . generics , "the main function cannot have generics") ; } if ! f . sig . inputs . is_empty () { bail_span ! (& f . sig . inputs , "the main function cannot have arguments") ; } let r#return = f . sig . output ; f . sig . output = ReturnType :: Default ; let body = f . block . as_ref () ; let wasm_bindgen = & program . wasm_bindgen ; let wasm_bindgen_futures = & program . wasm_bindgen_futures ; if f . sig . asyncness . take () . is_some () { * f . block = syn :: parse2 (quote :: quote ! { { async fn __wasm_bindgen_generated_main () # r#return # body # wasm_bindgen_futures :: spawn_local (async move { use # wasm_bindgen :: __rt :: Main ; let __ret = __wasm_bindgen_generated_main () ; (& mut & mut & mut # wasm_bindgen :: __rt :: MainWrapper (Some (__ret . await))) . __wasm_bindgen_main () } ,) } }) . unwrap () ; } else { * f . block = syn :: parse2 (quote :: quote ! { { fn __wasm_bindgen_generated_main () # r#return # body use # wasm_bindgen :: __rt :: Main ; let __ret = __wasm_bindgen_generated_main () ; (& mut & mut & mut # wasm_bindgen :: __rt :: MainWrapper (Some (__ret))) . __wasm_bindgen_main () } }) . unwrap () ; } f . to_tokens (tokens) ; Ok (()) }
};
}
