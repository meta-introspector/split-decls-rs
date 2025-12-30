// Generated macro for impl_30 (impl)
macro_rules! Depcrate_diagnosticimpl_30 {
() => {
// Module: crate::diagnostic
// Provides: {"impl_30"}
// Dependencies: {}
# [cfg (feature = "syn-error")] impl From < syn :: Error > for Diagnostic { fn from (err : syn :: Error) -> Self { use proc_macro2 :: { Delimiter , TokenTree } ; fn gut_error (ts : & mut impl Iterator < Item = TokenTree >) -> Option < (SpanRange , String) > { let start_span = ts . next () ? . span () ; ts . next () . expect (":1") ; ts . next () . expect ("core") ; ts . next () . expect (":2") ; ts . next () . expect (":3") ; ts . next () . expect ("compile_error") ; ts . next () . expect ("!") ; let lit = match ts . next () . unwrap () { TokenTree :: Group (group) => { if group . delimiter () == Delimiter :: Parenthesis || group . delimiter () == Delimiter :: Bracket { ts . next () . unwrap () ; } match group . stream () . into_iter () . next () . unwrap () { TokenTree :: Literal (lit) => lit , _ => unreachable ! ("") , } } _ => unreachable ! ("") , } ; let last = lit . span () ; let mut msg = lit . to_string () ; msg . pop () ; msg . remove (0) ; Some ((SpanRange { first : start_span , last , } , msg ,)) } let mut ts = err . to_compile_error () . into_iter () ; let (span_range , msg) = gut_error (& mut ts) . unwrap () ; let mut res = Diagnostic :: spanned_range (span_range , Level :: Error , msg) ; while let Some ((span_range , msg)) = gut_error (& mut ts) { res = res . span_range_error (span_range , msg) ; } res } }
};
}
