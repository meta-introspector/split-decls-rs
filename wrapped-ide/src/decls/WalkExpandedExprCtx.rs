macro_rules! WalkExpandedExprCtx {
    () => {
        # [doc = " Preorder walk all the expression's child expressions."] # [doc = " For macro calls, the callback will be called on the expanded expressions after"] # [doc = " visiting the macro call itself."] struct WalkExpandedExprCtx < 'a > { sema : & 'a Semantics < 'a , RootDatabase > , depth : usize , check_ctx : & 'static dyn Fn (& ast :: Expr) -> bool , }
    };
}

WalkExpandedExprCtx!();