// Generated macro for Token (enum)
macro_rules! Depcrate_grammar_lexerToken {
() => {
// Module: crate::grammar_lexer
// Provides: {"Token"}
// Dependencies: {}
# [derive (Debug)] # [derive (Eq)] # [derive (PartialEq)] # [derive (Clone)] pub enum Token { Equals , Ident (String) , Number (i32) , PlusSign , MinusSign , MultSign , DivideSign , ModuloSign , OutputCmd , NewLine , OpenParen , CloseParen , OpenBrace , CloseBrace , IfKeyword , ElseKeyword , WhileKeyword , LoopKeyword , Cmp (Comparator) , }
};
}
