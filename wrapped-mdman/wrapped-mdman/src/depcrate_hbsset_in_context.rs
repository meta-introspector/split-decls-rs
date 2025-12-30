// Generated macro for set_in_context (function)
macro_rules! Depcrate_hbsset_in_context {
() => {
// Module: crate::hbs
// Provides: {"set_in_context"}
// Dependencies: {}
# [doc = " Sets a variable to a value within the context."] fn set_in_context (rc : & mut RenderContext < '_ , '_ > , key : & str , value : serde_json :: Value) { let mut gctx = match rc . context () { Some (c) => (* c) . clone () , None => Context :: wraps (serde_json :: Value :: Object (serde_json :: Map :: new ())) . unwrap () , } ; if let serde_json :: Value :: Object (m) = gctx . data_mut () { m . insert (key . to_string () , value) ; rc . set_context (gctx) ; } else { panic ! ("expected object in context") ; } }
};
}
