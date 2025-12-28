macro_rules! deps {
    () => {
        Expr!();
    };
}

macro_rules! ExprField {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy , HashStable_Generic)] pub struct ExprField < 'hir > { # [stable_hasher (ignore)] pub hir_id : HirId , pub ident : Ident , pub expr : & 'hir Expr < 'hir > , pub span : Span , pub is_shorthand : bool , }
    };
}

ExprField!();