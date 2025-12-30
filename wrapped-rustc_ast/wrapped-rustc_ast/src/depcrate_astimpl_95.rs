// Generated macro for impl_95 (impl)
macro_rules! Depcrate_astimpl_95 {
() => {
// Module: crate::ast
// Provides: {"impl_95"}
// Dependencies: {}
impl Stmt { pub fn has_trailing_semicolon (& self) -> bool { match & self . kind { StmtKind :: Semi (_) => true , StmtKind :: MacCall (mac) => matches ! (mac . style , MacStmtStyle :: Semicolon) , _ => false , } } # [doc = " Converts a parsed `Stmt` to a `Stmt` with"] # [doc = " a trailing semicolon."] # [doc = ""] # [doc = " This only modifies the parsed AST struct, not the attached"] # [doc = " `LazyAttrTokenStream`. The parser is responsible for calling"] # [doc = " `ToAttrTokenStream::add_trailing_semi` when there is actually"] # [doc = " a semicolon in the tokenstream."] pub fn add_trailing_semicolon (mut self) -> Self { self . kind = match self . kind { StmtKind :: Expr (expr) => StmtKind :: Semi (expr) , StmtKind :: MacCall (mut mac) => { mac . style = MacStmtStyle :: Semicolon ; StmtKind :: MacCall (mac) } kind => kind , } ; self } pub fn is_item (& self) -> bool { matches ! (self . kind , StmtKind :: Item (_)) } pub fn is_expr (& self) -> bool { matches ! (self . kind , StmtKind :: Expr (_)) } }
};
}
