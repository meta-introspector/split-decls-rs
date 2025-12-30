// Generated macro for MacroParse (trait)
macro_rules! Depcrate_parserMacroParse {
() => {
// Module: crate::parser
// Provides: {"MacroParse"}
// Dependencies: {}
pub (crate) trait MacroParse < Ctx > { # [doc = " Parse the contents of an object into our AST, with a context if necessary."] # [doc = ""] # [doc = " The context is used to have access to the attributes on `#[wasm_bindgen]`, and to allow"] # [doc = " writing to the output `TokenStream`."] fn macro_parse (self , program : & mut ast :: Program , context : Ctx) -> Result < () , Diagnostic > ; }
};
}
