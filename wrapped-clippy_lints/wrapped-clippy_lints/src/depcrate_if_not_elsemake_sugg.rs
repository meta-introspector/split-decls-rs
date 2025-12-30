// Generated macro for make_sugg (function)
macro_rules! Depcrate_if_not_elsemake_sugg {
() => {
// Module: crate::if_not_else
// Provides: {"make_sugg"}
// Dependencies: {}
fn make_sugg < 'a > (sess : & impl HasSession , cond_kind : & 'a ExprKind < 'a > , cond_inner : Span , els_span : Span , default : & 'a str , indent_relative_to : Option < Span > ,) -> String { let cond_inner_snip = snippet (sess , cond_inner , default) ; let els_snip = snippet (sess , els_span , default) ; let indent = indent_relative_to . and_then (| s | indent_of (sess , s)) ; let suggestion = match cond_kind { ExprKind :: Unary (UnOp :: Not , cond_rest) => { format ! ("if {} {} else {}" , snippet (sess , cond_rest . span , default) , els_snip , cond_inner_snip) } , ExprKind :: Binary (_ , lhs , rhs) => { let lhs_snip = snippet (sess , lhs . span , default) ; let rhs_snip = snippet (sess , rhs . span , default) ; format ! ("if {lhs_snip} == {rhs_snip} {els_snip} else {cond_inner_snip}") } , _ => String :: new () , } ; reindent_multiline (& suggestion , true , indent) }
};
}
