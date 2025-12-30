// Generated macro for impl_203 (impl)
macro_rules! Depcrate_parser_stateimpl_203 {
() => {
// Module: crate::parser_state
// Provides: {"impl_203"}
// Dependencies: {}
impl Display for ParsingToken { fn fmt (& self , f : & mut Formatter < '_ >) -> core :: fmt :: Result { match self { ParsingToken :: Sensitive { token } => write ! (f , "{token}") , ParsingToken :: Insensitive { token } => write ! (f , "{}" , token . to_uppercase ()) , ParsingToken :: Range { start , end } => write ! (f , "{start}..{end}") , ParsingToken :: BuiltInRule => write ! (f , "BUILTIN_RULE") , } } }
};
}
