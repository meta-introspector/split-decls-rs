macro_rules! requires_comma_to_be_match_arm {
    () => {
        pub (crate) fn requires_comma_to_be_match_arm (mut expr : & Expr) -> bool { loop { match expr { # ! [cfg_attr (all (test , exhaustive) , deny (non_exhaustive_omitted_patterns))] Expr :: If (_) | Expr :: Match (_) | Expr :: Block (_) | Expr :: Unsafe (_) | Expr :: While (_) | Expr :: Loop (_) | Expr :: ForLoop (_) | Expr :: TryBlock (_) | Expr :: Const (_) => return false , Expr :: Array (_) | Expr :: Assign (_) | Expr :: Async (_) | Expr :: Await (_) | Expr :: Binary (_) | Expr :: Break (_) | Expr :: Call (_) | Expr :: Cast (_) | Expr :: Closure (_) | Expr :: Continue (_) | Expr :: Field (_) | Expr :: Index (_) | Expr :: Infer (_) | Expr :: Let (_) | Expr :: Lit (_) | Expr :: Macro (_) | Expr :: MethodCall (_) | Expr :: Paren (_) | Expr :: Path (_) | Expr :: Range (_) | Expr :: RawAddr (_) | Expr :: Reference (_) | Expr :: Repeat (_) | Expr :: Return (_) | Expr :: Struct (_) | Expr :: Try (_) | Expr :: Tuple (_) | Expr :: Unary (_) | Expr :: Yield (_) | Expr :: Verbatim (_) => return true , Expr :: Group (group) => expr = & group . expr , _ => return true , } } }
    };
}

requires_comma_to_be_match_arm!()