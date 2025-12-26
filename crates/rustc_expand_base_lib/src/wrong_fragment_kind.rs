use rustc_span::Span;
use rustc_errors::DiagMessage;
use rustc_ast;

pub struct WrongFragmentKind<'a> {
    pub span: Span,
    pub kind: &'a str,
    pub name: &'a rustc_ast::Path,
}
