// Generated macro for Token (enum)
macro_rules! Depcrate_parser_lexerToken {
() => {
// Module: crate::parser::lexer
// Provides: {"Token"}
// Dependencies: {}
# [doc = " A single token in the input source"] # [expect (missing_docs , reason = "self-explanatory")] # [derive (Clone , Copy , Debug , Display , Eq , PartialEq)] pub enum Token < 'a > { Name (& 'a str) , Scalar (ScalarToken < 'a >) , # [display ("!")] ExclamationMark , # [display ("$")] Dollar , # [display ("(")] ParenOpen , # [display (")")] ParenClose , # [display ("[")] BracketOpen , # [display ("]")] BracketClose , # [display ("{{")] CurlyOpen , # [display ("}}")] CurlyClose , # [display ("...")] Ellipsis , # [display (":")] Colon , # [display ("=")] Equals , # [display ("@")] At , # [display ("|")] Pipe , # [display ("End of file")] EndOfFile , }
};
}
