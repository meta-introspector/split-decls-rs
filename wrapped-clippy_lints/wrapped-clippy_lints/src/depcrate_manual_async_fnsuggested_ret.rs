// Generated macro for suggested_ret (function)
macro_rules! Depcrate_manual_async_fnsuggested_ret {
() => {
// Module: crate::manual_async_fn
// Provides: {"suggested_ret"}
// Dependencies: {}
fn suggested_ret (cx : & LateContext < '_ > , output : & Ty < '_ >) -> Option < (& 'static str , String) > { if let TyKind :: Tup ([]) = output . kind { let sugg = "remove the return type" ; Some ((sugg , String :: new ())) } else { let sugg = "return the output of the future directly" ; output . span . get_source_text (cx) . map (| src | (sugg , format ! (" -> {src}"))) } }
};
}
