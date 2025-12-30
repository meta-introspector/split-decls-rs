// Generated macro for impl_196 (impl)
macro_rules! Depcrate_parserimpl_196 {
() => {
// Module: crate::parser
// Provides: {"impl_196"}
// Dependencies: {}
impl ConvertToAst < (BindgenAttrs , Vec < FnArgAttrs >) > for syn :: ItemFn { type Target = ast :: Function ; fn convert (self , (attrs , args_attrs) : (BindgenAttrs , Vec < FnArgAttrs >) ,) -> Result < Self :: Target , Diagnostic > { match self . vis { syn :: Visibility :: Public (_) => { } _ if attrs . start () . is_some () => { } _ => bail_span ! (self , "can only #[wasm_bindgen] public functions") , } if self . sig . constness . is_some () { bail_span ! (self . sig . constness , "can only #[wasm_bindgen] non-const functions") ; } let (mut ret , _) = function_from_decl (& self . sig . ident , & attrs , self . sig . clone () , self . attrs , self . vis , FunctionPosition :: Free , Some (args_attrs) ,) ? ; attrs . check_used () ; if is_js_keyword (& ret . name) && ret . name != "default" { ret . name = format ! ("_{}" , ret . name) ; } Ok (ret) } }
};
}
