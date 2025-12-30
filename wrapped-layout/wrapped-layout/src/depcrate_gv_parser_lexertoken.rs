// Generated macro for Token (enum)
macro_rules! Depcrate_gv_parser_lexerToken {
() => {
// Module: crate::gv::parser::lexer
// Provides: {"Token"}
// Dependencies: {}
# [derive (Debug , Clone)] pub enum Token { EOF , Identifier (String) , GraphKW , NodeKW , EdgeKW , DigraphKW , StrictKW , SubgraphKW , Equal , Colon , Comma , Semicolon , ArrowRight , ArrowLine , OpenBracket , CloseBracket , OpenBrace , CloseBrace , Error (usize) , }
};
}
