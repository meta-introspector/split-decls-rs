// Generated macro for expand (function)
macro_rules! Depcrateexpand {
() => {
// Module: crate
// Provides: {"expand"}
// Dependencies: {}
# [doc = " Takes the parsed input from a `#[wasm_bindgen]` macro and returns the generated bindings"] pub fn expand (attr : TokenStream , input : TokenStream) -> Result < TokenStream , Diagnostic > { parser :: reset_attrs_used () ; let item = syn :: parse2 :: < syn :: Item > (input) ? ; if let syn :: Item :: Struct (s) = item { let opts : BindgenAttrs = syn :: parse2 (attr . clone ()) ? ; let wasm_bindgen = opts . wasm_bindgen () . cloned () . unwrap_or_else (| | syn :: parse_quote ! { :: wasm_bindgen }) ; let item = quote ! { # [derive (# wasm_bindgen :: __rt :: BindgenedStruct)] # [wasm_bindgen (# attr)] # s } ; return Ok (item) ; } let opts = syn :: parse2 (attr) ? ; let mut tokens = proc_macro2 :: TokenStream :: new () ; let mut program = ast :: Program :: default () ; item . macro_parse (& mut program , (Some (opts) , & mut tokens)) ? ; program . try_to_tokens (& mut tokens) ? ; parser :: check_unused_attrs (& mut tokens) ; Ok (tokens) }
};
}
