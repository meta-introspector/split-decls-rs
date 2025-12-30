// Generated macro for TokenTree (enum)
macro_rules! DepcrateTokenTree {
() => {
// Module: crate
// Provides: {"TokenTree"}
// Dependencies: {}
# [doc = " Similar to `tokenstream::TokenTree`, except that `Sequence`, `MetaVar`, `MetaVarDecl`, and"] # [doc = " `MetaVarExpr` are \"first-class\" token trees. Useful for parsing macros."] # [derive (Debug , PartialEq , Encodable , Decodable)] pub enum TokenTree { # [doc = " A token. Unlike `tokenstream::TokenTree::Token` this lacks a `Spacing`."] # [doc = " See the comments about `Spacing` in the `transcribe` function."] Token (Token) , # [doc = " A delimited sequence, e.g. `($e:expr)` (RHS) or `{ $e }` (LHS)."] Delimited (DelimSpan , DelimSpacing , Delimited) , # [doc = " A kleene-style repetition sequence, e.g. `$($e:expr)*` (RHS) or `$($e),*` (LHS)."] Sequence (DelimSpan , SequenceRepetition) , # [doc = " e.g., `$var`. The span covers the leading dollar and the ident. (The span within the ident"] # [doc = " only covers the ident, e.g. `var`.)"] MetaVar (Span , Ident) , # [doc = " e.g., `$var:expr`. Only appears on the LHS."] MetaVarDecl { span : Span , # [doc = " Name to bind."] name : Ident , # [doc = " The fragment specifier."] kind : NonterminalKind , } , # [doc = " A meta-variable expression inside ${...}."] MetaVarExpr (DelimSpan , MetaVarExpr) , }
};
}
