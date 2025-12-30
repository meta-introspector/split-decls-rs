// Generated macro for find_assert_args (function)
macro_rules! Depcrate_macrosfind_assert_args {
() => {
// Module: crate::macros
// Provides: {"find_assert_args"}
// Dependencies: {}
# [doc = " Finds the arguments of an `assert!` or `debug_assert!` macro call within the macro expansion"] pub fn find_assert_args < 'a > (cx : & LateContext < '_ > , expr : & 'a Expr < 'a > , expn : ExpnId ,) -> Option < (& 'a Expr < 'a > , PanicExpn < 'a >) > { find_assert_args_inner (cx , expr , expn) . map (| ([e] , mut p) | { if let PanicExpn :: Str (_) = p { p = PanicExpn :: Empty ; } (e , p) }) }
};
}
