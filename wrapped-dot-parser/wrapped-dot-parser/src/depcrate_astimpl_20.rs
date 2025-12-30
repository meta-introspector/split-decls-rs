// Generated macro for impl_20 (impl)
macro_rules! Depcrate_astimpl_20 {
() => {
// Module: crate::ast
// Provides: {"impl_20"}
// Dependencies: {}
impl Display for ParseError < '_ > { fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , std :: fmt :: Error > { match self { ParseError :: ExpectRule { expect , found } => { let expected = expect . iter () . fold (String :: new () , | acc , r | format ! ("{}\"{:?}\", " , acc , r)) ; write ! (f , "Expect one rule of {}but \"{:?}\" found." , expected , found) ? ; } ParseError :: MissingPair { parent , expect } => { let mut expected = expect . iter () . fold (String :: new () , | acc , r | format ! ("{}\"{:?}\", " , acc , r)) ; expected . pop () ; expected . pop () ; write ! (f , "The Pair:\n{}\nterminates early. Expected one of {}." , parent . as_str () , expected) ? ; } } Ok (()) } }
};
}
