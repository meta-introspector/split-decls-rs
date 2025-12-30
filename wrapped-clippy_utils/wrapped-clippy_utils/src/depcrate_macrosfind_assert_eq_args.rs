// Generated macro for find_assert_eq_args (function)
macro_rules! Depcrate_macrosfind_assert_eq_args {
() => {
// Module: crate::macros
// Provides: {"find_assert_eq_args"}
// Dependencies: {}
# [doc = " Finds the arguments of an `assert_eq!` or `debug_assert_eq!` macro call within the macro"] # [doc = " expansion"] pub fn find_assert_eq_args < 'a > (cx : & LateContext < '_ > , expr : & 'a Expr < 'a > , expn : ExpnId ,) -> Option < (& 'a Expr < 'a > , & 'a Expr < 'a > , PanicExpn < 'a >) > { find_assert_args_inner (cx , expr , expn) . map (| ([a , b] , p) | (a , b , p)) }
};
}
