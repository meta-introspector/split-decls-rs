macro_rules! deps {
    () => {
        BreakableKind!();
    };
}

macro_rules! PathExprCtx {
    () => {
        deps!();
        # [derive (Debug , PartialEq , Eq)] pub (crate) struct PathExprCtx < 'db > { pub (crate) in_block_expr : bool , pub (crate) in_breakable : Option < BreakableKind > , pub (crate) after_if_expr : bool , pub (crate) before_else_kw : bool , # [doc = " Whether this expression is the direct condition of an if or while expression"] pub (crate) in_condition : bool , pub (crate) incomplete_let : bool , pub (crate) after_incomplete_let : bool , pub (crate) in_value : bool , pub (crate) ref_expr_parent : Option < ast :: RefExpr > , pub (crate) after_amp : bool , # [doc = " The surrounding RecordExpression we are completing a functional update"] pub (crate) is_func_update : Option < ast :: RecordExpr > , pub (crate) self_param : Option < hir :: SelfParam > , pub (crate) innermost_ret_ty : Option < hir :: Type < 'db > > , pub (crate) innermost_breakable_ty : Option < hir :: Type < 'db > > , pub (crate) impl_ : Option < ast :: Impl > , # [doc = " Whether this expression occurs in match arm guard position: before the"] # [doc = " fat arrow token"] pub (crate) in_match_guard : bool , }
    };
}

PathExprCtx!();