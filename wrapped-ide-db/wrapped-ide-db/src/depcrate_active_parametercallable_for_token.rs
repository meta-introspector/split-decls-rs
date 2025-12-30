// Generated macro for callable_for_token (function)
macro_rules! Depcrate_active_parametercallable_for_token {
() => {
// Module: crate::active_parameter
// Provides: {"callable_for_token"}
// Dependencies: {}
# [doc = " Returns a [`hir::Callable`] this token is a part of and its argument index of said callable."] pub fn callable_for_token < 'db > (sema : & Semantics < 'db , RootDatabase > , token : SyntaxToken ,) -> Option < (hir :: Callable < 'db > , Option < usize >) > { let offset = token . text_range () . start () ; let parent = token . parent () ? ; let calling_node = parent . ancestors () . filter_map (ast :: CallableExpr :: cast) . find (| it | it . arg_list () . is_some_and (| it | it . syntax () . text_range () . contains (offset))) ? ; callable_for_node (sema , & calling_node , offset) }
};
}
