// Generated macro for impl_float (macro)
macro_rules! Depcrate_parseimpl_float {
() => {
// Module: crate::parse
// Provides: {"impl_float"}
// Dependencies: {}
macro_rules ! impl_float { ($ wrap : ident ($ ty : ty : $ bits : expr)) => { impl Float for $ ty { fn parse (float : & str) -> Result < Self > { <$ ty >:: from_str (float) . map_err (| _ | Error :: ExpectedFloat) } fn try_from_parsed_float (parsed : ParsedFloat , ron : & str) -> Result < Self > { match parsed { ParsedFloat ::$ wrap (v) => Ok (v) , _ => Err (Error :: InvalidValueForType { expected : format ! ("a {}-bit floating point number" , $ bits ,) , found : String :: from (ron) , }) , } } } } ; ($ ($ wraps : ident ($ tys : ty : $ bits : expr)) *) => { $ (impl_float ! ($ wraps ($ tys : $ bits)) ;) * } ; }
};
}
