// Generated macro for parse_signature (function)
macro_rules! Depcrate_parseparse_signature {
() => {
// Module: crate::parse
// Provides: {"parse_signature"}
// Dependencies: {}
fn parse_signature (input : & mut Peekable < impl Iterator < Item = TokenTree > >) -> Vec < TokenTree > { let mut sig = Vec :: new () ; loop { match input . peek () { Some (TokenTree :: Group (ref group)) if group . delimiter () == Delimiter :: Brace => { return sig ; } None => return sig , _ => sig . push (input . next () . unwrap ()) , } } }
};
}
