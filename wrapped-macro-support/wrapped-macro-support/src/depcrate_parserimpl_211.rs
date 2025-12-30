// Generated macro for impl_211 (impl)
macro_rules! Depcrate_parserimpl_211 {
() => {
// Module: crate::parser
// Provides: {"impl_211"}
// Dependencies: {}
impl MacroParse < BindgenAttrs > for syn :: ItemConst { fn macro_parse (self , program : & mut ast :: Program , opts : BindgenAttrs) -> Result < () , Diagnostic > { if opts . typescript_custom_section () . is_none () { bail_span ! (self , "#[wasm_bindgen] will not work on constants unless you are defining a #[wasm_bindgen(typescript_custom_section)].") ; } let typescript_custom_section = match get_expr (& self . expr) { syn :: Expr :: Lit (syn :: ExprLit { lit : syn :: Lit :: Str (litstr) , .. }) => ast :: LitOrExpr :: Lit (litstr . value ()) , expr => ast :: LitOrExpr :: Expr (expr . clone ()) , } ; program . typescript_custom_sections . push (typescript_custom_section) ; opts . check_used () ; Ok (()) } }
};
}
