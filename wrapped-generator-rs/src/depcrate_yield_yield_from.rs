// Generated macro for yield_from (function)
macro_rules! Depcrate_yield_yield_from {
() => {
// Module: crate::yield_
// Provides: {"yield_from"}
// Dependencies: {}
# [doc = " `yield_from`"] # [deprecated (since = "0.6.18" , note = "please use `scope` version instead")] pub fn yield_from < A : Any , T : Any > (mut g : Generator < A , T >) -> Option < A > { let env = ContextStack :: current () ; let context = env . top () ; let mut p = context . get_para () ; while unlikely (! g . is_done ()) { match g . raw_send (p) { None => return None , Some (r) => raw_yield (& env , context , r) , } p = context . get_para () ; } drop (g) ; p }
};
}
