macro_rules! deps {
    () => {
        DeclOrigin!();
        Declaration!();
    };
}

macro_rules! impl_224 {
    () => {
        deps!();
        impl < 'a > From < (& 'a hir :: LetExpr < 'a > , HirId) > for Declaration < 'a > { fn from ((let_expr , hir_id) : (& 'a hir :: LetExpr < 'a > , HirId)) -> Self { let hir :: LetExpr { pat , ty , span , init , recovered : _ } = * let_expr ; Declaration { hir_id , pat , ty , span , init : Some (init) , origin : DeclOrigin :: LetExpr } } }
    };
}

impl_224!();