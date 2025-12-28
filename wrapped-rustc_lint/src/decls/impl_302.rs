macro_rules! deps {
    () => {
        LateContext!();
        SymbolInternStringLiteralDiag!();
    };
}

macro_rules! impl_302 {
    () => {
        deps!();
        impl < 'tcx > LateLintPass < 'tcx > for SymbolInternStringLiteral { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx rustc_hir :: Expr < 'tcx >) { if let hir :: ExprKind :: Call (path , [arg]) = expr . kind && let hir :: ExprKind :: Path (ref qpath) = path . kind && let Some (def_id) = cx . qpath_res (qpath , path . hir_id) . opt_def_id () && cx . tcx . is_diagnostic_item (sym :: SymbolIntern , def_id) && let hir :: ExprKind :: Lit (kind) = arg . kind && let rustc_ast :: LitKind :: Str (_ , _) = kind . node { cx . emit_span_lint (SYMBOL_INTERN_STRING_LITERAL , kind . span , SymbolInternStringLiteralDiag ,) ; } } }
    };
}

impl_302!()