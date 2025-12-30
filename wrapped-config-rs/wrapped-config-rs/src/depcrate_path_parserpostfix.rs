// Generated macro for postfix (function)
macro_rules! Depcrate_path_parserpostfix {
() => {
// Module: crate::path::parser
// Provides: {"postfix"}
// Dependencies: {}
fn postfix (i : & mut & str) -> ModalResult < Postfix > { dispatch ! { any ; '[' => cut_err (seq ! (integer . map (Postfix :: Index) , _ : ']' . context (StrContext :: Expected (StrContextValue :: CharLiteral (']'))) ,) . map (| (i ,) | i) . context (StrContext :: Label ("subscript"))) , '.' => cut_err (ident . map (Postfix :: Key)) , _ => cut_err (fail . context (StrContext :: Label ("postfix")) . context (StrContext :: Expected (StrContextValue :: CharLiteral ('['))) . context (StrContext :: Expected (StrContextValue :: CharLiteral ('.')))) , } . parse_next (i) }
};
}
