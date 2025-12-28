macro_rules! path_is_static_mut {
    () => {
        fn path_is_static_mut (mut expr : & hir :: Expr < '_ > , mut err_span : Span) -> Option < Span > { if err_span . from_expansion () { err_span = expr . span ; } while let hir :: ExprKind :: Field (e , _) = expr . kind { expr = e ; } if let hir :: ExprKind :: Path (qpath) = expr . kind && let hir :: QPath :: Resolved (_ , path) = qpath && let hir :: def :: Res :: Def (def_kind , _) = path . res && let hir :: def :: DefKind :: Static { safety : _ , mutability : Mutability :: Mut , nested : false } = def_kind { return Some (err_span) ; } None }
    };
}

path_is_static_mut!();