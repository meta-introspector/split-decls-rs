use rustc_span::Span;
use rustc_errors::DiagMessage;
use rustc_span::Symbol;
use rustc_hir::limit::Limit;

pub struct RecursionLimitReached {
    pub span: Span,
    pub descr: String,
    pub suggested_limit: Limit,
    pub crate_name: Symbol,
}
