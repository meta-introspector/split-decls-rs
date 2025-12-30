// Generated macro for impl_212 (impl)
macro_rules! Depcrate_parserimpl_212 {
() => {
// Module: crate::parser
// Provides: {"impl_212"}
// Dependencies: {}
impl MacroParse < BindgenAttrs > for syn :: ItemForeignMod { fn macro_parse (self , program : & mut ast :: Program , opts : BindgenAttrs) -> Result < () , Diagnostic > { let mut errors = Vec :: new () ; if let Some (other) = self . abi . name . filter (| l | l . value () != "C") { errors . push (err_span ! (other , "only foreign mods with the `C` ABI are allowed")) ; } let js_namespace = opts . js_namespace () . map (| (s , _) | s) ; let module = module_from_opts (program , & opts) . map_err (| e | errors . push (e)) . unwrap_or_default () ; for item in self . items . into_iter () { let ctx = ForeignItemCtx { module : module . clone () , js_namespace : js_namespace . clone () , } ; if let Err (e) = item . macro_parse (program , ctx) { errors . push (e) ; } } Diagnostic :: from_vec (errors) ? ; opts . check_used () ; Ok (()) } }
};
}
