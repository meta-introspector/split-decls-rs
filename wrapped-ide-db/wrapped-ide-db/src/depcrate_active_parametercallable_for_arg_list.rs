// Generated macro for callable_for_arg_list (function)
macro_rules! Depcrate_active_parametercallable_for_arg_list {
() => {
// Module: crate::active_parameter
// Provides: {"callable_for_arg_list"}
// Dependencies: {}
# [doc = " Returns a [`hir::Callable`] this token is a part of and its argument index of said callable."] pub fn callable_for_arg_list < 'db > (sema : & Semantics < 'db , RootDatabase > , arg_list : ast :: ArgList , at : TextSize ,) -> Option < (hir :: Callable < 'db > , Option < usize >) > { debug_assert ! (arg_list . syntax () . text_range () . contains (at)) ; let callable = arg_list . syntax () . parent () . and_then (ast :: CallableExpr :: cast) ? ; callable_for_node (sema , & callable , at) }
};
}
