// Generated macro for impl_321 (impl)
macro_rules! Depcrate_parser_lexerimpl_321 {
() => {
// Module: crate::parser::lexer
// Provides: {"impl_321"}
// Dependencies: {}
impl Deref for StringLiteral < '_ > { type Target = str ; fn deref (& self) -> & Self :: Target { match self { Self :: Quoted (s) => s , Self :: Block (s) => s , } } }
};
}
