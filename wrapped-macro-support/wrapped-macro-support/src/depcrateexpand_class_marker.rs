// Generated macro for expand_class_marker (function)
macro_rules! Depcrateexpand_class_marker {
() => {
// Module: crate
// Provides: {"expand_class_marker"}
// Dependencies: {}
# [doc = " Takes the parsed input from a `#[wasm_bindgen]` macro and returns the generated bindings"] pub fn expand_class_marker (attr : TokenStream , input : TokenStream ,) -> Result < TokenStream , Diagnostic > { parser :: reset_attrs_used () ; let mut item = syn :: parse2 :: < syn :: ImplItemFn > (input) ? ; let opts : ClassMarker = syn :: parse2 (attr) ? ; let mut program = ast :: Program :: default () ; item . macro_parse (& mut program , & opts) ? ; let mut tokens = proc_macro2 :: TokenStream :: new () ; tokens . append_all (item . attrs . iter () . filter (| attr | matches ! (attr . style , syn :: AttrStyle :: Outer)) ,) ; item . vis . to_tokens (& mut tokens) ; item . sig . to_tokens (& mut tokens) ; let mut err = None ; item . block . brace_token . surround (& mut tokens , | tokens | { if let Err (e) = program . try_to_tokens (tokens) { err = Some (e) ; } parser :: check_unused_attrs (tokens) ; tokens . append_all (item . attrs . iter () . filter (| attr | matches ! (attr . style , syn :: AttrStyle :: Inner (_))) ,) ; tokens . append_all (& item . block . stmts) ; }) ; if let Some (err) = err { return Err (err) ; } Ok (tokens) }
};
}
