// Generated macro for string_enum (function)
macro_rules! Depcrate_parserstring_enum {
() => {
// Module: crate::parser
// Provides: {"string_enum"}
// Dependencies: {}
fn string_enum (enum_ : syn :: ItemEnum , program : & mut ast :: Program , js_name : String , generate_typescript : bool , comments : Vec < String > , js_namespace : Option < Vec < String > > ,) -> Result < () , Diagnostic > { let mut variants = vec ! [] ; let mut variant_values = vec ! [] ; for v in enum_ . variants . iter () { let (_ , expr) = match & v . discriminant { Some (pair) => pair , None => { bail_span ! (v , "all variants of a string enum must have a string value") ; } } ; match get_expr (expr) { syn :: Expr :: Lit (syn :: ExprLit { attrs : _ , lit : syn :: Lit :: Str (str_lit) , }) => { variants . push (v . ident . clone ()) ; variant_values . push (str_lit . value ()) ; } expr => bail_span ! (expr , "enums with #[wasm_bindgen] cannot mix string and non-string values" ,) , } } program . imports . push (ast :: Import { module : None , js_namespace : None , reexport : None , kind : ast :: ImportKind :: Enum (ast :: StringEnum { vis : enum_ . vis , name : enum_ . ident , js_name , variants , variant_values , comments , rust_attrs : enum_ . attrs , generate_typescript , js_namespace , wasm_bindgen : program . wasm_bindgen . clone () , }) , }) ; Ok (()) }
};
}
