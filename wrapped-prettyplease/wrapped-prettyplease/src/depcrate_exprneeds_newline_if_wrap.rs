// Generated macro for needs_newline_if_wrap (function)
macro_rules! Depcrate_exprneeds_newline_if_wrap {
() => {
// Module: crate::expr
// Provides: {"needs_newline_if_wrap"}
// Dependencies: {}
fn needs_newline_if_wrap (expr : & Expr) -> bool { match expr { # ! [cfg_attr (all (test , exhaustive) , deny (non_exhaustive_omitted_patterns))] Expr :: Array (_) | Expr :: Async (_) | Expr :: Block (_) | Expr :: Break (ExprBreak { expr : None , .. }) | Expr :: Closure (_) | Expr :: Const (_) | Expr :: Continue (_) | Expr :: ForLoop (_) | Expr :: If (_) | Expr :: Infer (_) | Expr :: Lit (_) | Expr :: Loop (_) | Expr :: Macro (_) | Expr :: Match (_) | Expr :: Path (_) | Expr :: Range (ExprRange { end : None , .. }) | Expr :: Repeat (_) | Expr :: Return (ExprReturn { expr : None , .. }) | Expr :: Struct (_) | Expr :: TryBlock (_) | Expr :: Tuple (_) | Expr :: Unsafe (_) | Expr :: Verbatim (_) | Expr :: While (_) | Expr :: Yield (ExprYield { expr : None , .. }) => false , Expr :: Assign (_) | Expr :: Await (_) | Expr :: Binary (_) | Expr :: Cast (_) | Expr :: Field (_) | Expr :: Index (_) | Expr :: MethodCall (_) => true , Expr :: Break (ExprBreak { expr : Some (e) , .. }) | Expr :: Call (ExprCall { func : e , .. }) | Expr :: Group (ExprGroup { expr : e , .. }) | Expr :: Let (ExprLet { expr : e , .. }) | Expr :: Paren (ExprParen { expr : e , .. }) | Expr :: Range (ExprRange { end : Some (e) , .. }) | Expr :: RawAddr (ExprRawAddr { expr : e , .. }) | Expr :: Reference (ExprReference { expr : e , .. }) | Expr :: Return (ExprReturn { expr : Some (e) , .. }) | Expr :: Try (ExprTry { expr : e , .. }) | Expr :: Unary (ExprUnary { expr : e , .. }) | Expr :: Yield (ExprYield { expr : Some (e) , .. }) => needs_newline_if_wrap (e) , _ => false , } }
};
}
