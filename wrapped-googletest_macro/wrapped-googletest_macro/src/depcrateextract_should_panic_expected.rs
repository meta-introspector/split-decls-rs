// Generated macro for extract_should_panic_expected (function)
macro_rules! Depcrateextract_should_panic_expected {
() => {
// Module: crate
// Provides: {"extract_should_panic_expected"}
// Dependencies: {}
# [doc = " Extract the optional \"expected\" string literal from a `should_panic`"] # [doc = " attribute."] fn extract_should_panic_expected (attr : & Attribute) -> Option < String > { let Ok (name_value) = attr . parse_args :: < MetaNameValue > () else { return None ; } ; match name_value . value { Expr :: Lit (ExprLit { lit : Lit :: Str (expected) , .. }) if name_value . path . is_ident ("expected") => { Some (expected . value ()) } _ => None , } }
};
}
