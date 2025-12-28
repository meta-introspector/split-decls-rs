macro_rules! deps {
    () => {
        DeclOrigin!();
    };
}

macro_rules! Declaration {
    () => {
        deps!();
        # [doc = " A declaration is an abstraction of [hir::LetStmt] and [hir::LetExpr]."] # [doc = ""] # [doc = " It must have a hir_id, as this is how we connect gather_locals to the check functions."] pub (super) struct Declaration < 'a > { pub hir_id : HirId , pub pat : & 'a hir :: Pat < 'a > , pub ty : Option < & 'a hir :: Ty < 'a > > , pub span : Span , pub init : Option < & 'a hir :: Expr < 'a > > , pub origin : DeclOrigin < 'a > , }
    };
}

Declaration!()