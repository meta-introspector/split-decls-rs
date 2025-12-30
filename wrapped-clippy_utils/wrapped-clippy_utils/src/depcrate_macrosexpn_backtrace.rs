// Generated macro for expn_backtrace (function)
macro_rules! Depcrate_macrosexpn_backtrace {
() => {
// Module: crate::macros
// Provides: {"expn_backtrace"}
// Dependencies: {}
# [doc = " Returns an iterator of expansions that created the given span"] pub fn expn_backtrace (mut span : Span) -> impl Iterator < Item = (ExpnId , ExpnData) > { std :: iter :: from_fn (move | | { let ctxt = span . ctxt () ; if ctxt == SyntaxContext :: root () { return None ; } let expn = ctxt . outer_expn () ; let data = expn . expn_data () ; span = data . call_site ; Some ((expn , data)) }) }
};
}
