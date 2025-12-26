use rustc_span::{Span};
use lib_matcher_loc::MatcherLoc;
use lib_token_tree::TokenTree;

pub enum MacroRule {
    /// A function-style rule, for use with `m!()`
    Func { lhs: Vec<MatcherLoc>, lhs_span: Span, rhs: TokenTree },
    /// An attr rule, for use with `#[m]`
    Attr {
        unsafe_rule: bool,
        args: Vec<MatcherLoc>,
        args_span: Span,
        body: Vec<MatcherLoc>,
        body_span: Span,
        rhs: TokenTree,
    },
    /// A derive rule, for use with `#[m]`
    Derive { body: Vec<MatcherLoc>, body_span: Span, rhs: TokenTree },
}