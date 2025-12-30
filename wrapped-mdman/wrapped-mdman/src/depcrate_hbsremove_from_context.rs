// Generated macro for remove_from_context (function)
macro_rules! Depcrate_hbsremove_from_context {
() => {
// Module: crate::hbs
// Provides: {"remove_from_context"}
// Dependencies: {}
# [doc = " Removes a variable from the context."] fn remove_from_context (rc : & mut RenderContext < '_ , '_ > , key : & str) { let gctx = rc . context () . expect ("cannot remove from null context") ; let mut gctx = (* gctx) . clone () ; if let serde_json :: Value :: Object (m) = gctx . data_mut () { m . remove (key) ; rc . set_context (gctx) ; } else { panic ! ("expected object in context") ; } }
};
}
