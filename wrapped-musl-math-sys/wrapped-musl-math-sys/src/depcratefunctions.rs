// Generated macro for functions (macro)
macro_rules! Depcratefunctions {
() => {
// Module: crate
// Provides: {"functions"}
// Dependencies: {}
# [doc = " Macro for creating bindings and exposing a safe function (since the implementations have no"] # [doc = " preconditions). Included functions must have correct signatures, otherwise this will be"] # [doc = " unsound."] macro_rules ! functions { ($ ($ (# [$ meta : meta]) * $ pfx_name : ident : $ name : ident ($ ($ arg : ident : $ aty : ty) ,+) -> $ rty : ty ;) *) => { unsafe extern "C" { $ (fn $ pfx_name ($ ($ arg : $ aty) ,+) -> $ rty ;) * } $ ($ (# [$ meta]) * pub fn $ name ($ ($ arg : $ aty) ,+) -> $ rty { unsafe { $ pfx_name ($ ($ arg) ,+) } }) * # [cfg (test)] mod tests { use super ::*; use test_support :: CallTest ; $ (functions ! (@ single_test $ name ($ ($ arg : $ aty) ,+) -> $ rty) ;) * } } ; (@ single_test $ name : ident ($ ($ arg : ident : $ aty : ty) ,+) -> $ rty : ty) => { # [test] fn $ name () { < fn ($ ($ aty) ,+) -> $ rty >:: check (super ::$ name) ; } } ; }
};
}
