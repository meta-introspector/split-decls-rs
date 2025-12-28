macro_rules! TypeLimits {
    () => {
        # [derive (Copy , Clone , Default)] pub (crate) struct TypeLimits { # [doc = " Id of the last visited negated expression"] negated_expr_id : Option < hir :: HirId > , # [doc = " Span of the last visited negated expression"] negated_expr_span : Option < Span > , }
    };
}

TypeLimits!();