macro_rules! CastCheck {
    () => {
        # [doc = " Reifies a cast check to be checked once we have full type information for"] # [doc = " a function context."] # [derive (Debug)] pub (crate) struct CastCheck < 'tcx > { # [doc = " The expression whose value is being casted"] expr : & 'tcx hir :: Expr < 'tcx > , # [doc = " The source type for the cast expression"] expr_ty : Ty < 'tcx > , expr_span : Span , # [doc = " The target type. That is, the type we are casting to."] cast_ty : Ty < 'tcx > , cast_span : Span , span : Span , }
    };
}

CastCheck!();