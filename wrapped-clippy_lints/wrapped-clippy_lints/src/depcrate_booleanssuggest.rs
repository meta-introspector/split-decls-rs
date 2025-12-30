// Generated macro for suggest (function)
macro_rules! Depcrate_booleanssuggest {
() => {
// Module: crate::booleans
// Provides: {"suggest"}
// Dependencies: {}
fn suggest (cx : & LateContext < '_ > , msrv : Msrv , suggestion : & Bool , terminals : & [& Expr < '_ >]) -> String { let mut suggest_context = SuggestContext { terminals , cx , msrv , output : String :: new () , } ; suggest_context . recurse (suggestion) ; suggest_context . output }
};
}
