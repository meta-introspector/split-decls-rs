macro_rules! deps {
    () => {
        PatExprKind!();
    };
}

macro_rules! PatExpr {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy , HashStable_Generic)] pub struct PatExpr < 'hir > { # [stable_hasher (ignore)] pub hir_id : HirId , pub span : Span , pub kind : PatExprKind < 'hir > , }
    };
}

PatExpr!()