// Generated macro for impl_integer (macro)
macro_rules! Depcrate_parseimpl_integer {
() => {
// Module: crate::parse
// Provides: {"impl_integer"}
// Dependencies: {}
macro_rules ! impl_integer { ($ wrap : ident ($ ty : ty)) => { impl Integer for $ ty { fn parse (parser : & mut Parser , sign : i8) -> Result < Self > { parser . parse_integer (sign) } fn try_from_parsed_integer (parsed : ParsedInteger , ron : & str) -> Result < Self > { match parsed { ParsedInteger ::$ wrap (v) => Ok (v) , _ => Err (Error :: InvalidValueForType { expected : format ! ("a{} {}-bit {}signed integer" , if <$ ty >:: BITS == 8 { "n" } else { "n" } , <$ ty >:: BITS , if <$ ty >:: MIN == 0 { "un" } else { "" } ,) , found : String :: from (ron) , }) , } } } } ; ($ ($ wraps : ident ($ tys : ty)) *) => { $ (impl_integer ! ($ wraps ($ tys)) ;) * } ; }
};
}
