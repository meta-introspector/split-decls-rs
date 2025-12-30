// Generated macro for ident (function)
macro_rules! Depcrate_path_parserident {
() => {
// Module: crate::path::parser
// Provides: {"ident"}
// Dependencies: {}
fn ident (i : & mut & str) -> ModalResult < String > { take_while (1 .. , ('a' ..= 'z' , 'A' ..= 'Z' , '0' ..= '9' , '_' , '-')) . map (ToOwned :: to_owned) . context (StrContext :: Label ("identifier")) . context (StrContext :: Expected (StrContextValue :: Description ("ASCII alphanumeric" ,))) . context (StrContext :: Expected (StrContextValue :: CharLiteral ('_'))) . context (StrContext :: Expected (StrContextValue :: CharLiteral ('-'))) . parse_next (i) }
};
}
