macro_rules! deps {
    () => {
        HirTypedAnn!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl < 'tcx > pprust_hir :: PpAnn for HirTypedAnn < 'tcx > { fn nested (& self , state : & mut pprust_hir :: State < '_ > , nested : pprust_hir :: Nested) { let old_maybe_typeck_results = self . maybe_typeck_results . get () ; if let pprust_hir :: Nested :: Body (id) = nested { self . maybe_typeck_results . set (Some (self . tcx . typeck_body (id))) ; } self . tcx . nested (state , nested) ; self . maybe_typeck_results . set (old_maybe_typeck_results) ; } fn pre (& self , s : & mut pprust_hir :: State < '_ > , node : pprust_hir :: AnnNode < '_ >) { if let pprust_hir :: AnnNode :: Expr (_) = node { s . popen () ; } } fn post (& self , s : & mut pprust_hir :: State < '_ > , node : pprust_hir :: AnnNode < '_ >) { if let pprust_hir :: AnnNode :: Expr (expr) = node { let typeck_results = self . maybe_typeck_results . get () . or_else (| | { self . tcx . hir_maybe_body_owned_by (expr . hir_id . owner . def_id) . map (| body_id | self . tcx . typeck_body (body_id . id ())) }) ; if let Some (typeck_results) = typeck_results { s . s . space () ; s . s . word ("as") ; s . s . space () ; s . s . word (typeck_results . expr_ty (expr) . to_string ()) ; } s . pclose () ; } } }
    };
}

impl_19!()