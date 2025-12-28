macro_rules! deps {
    () => {
        FindClosureArg!();
    };
}

macro_rules! impl_200 {
    () => {
        deps!();
        impl < 'tcx > Visitor < 'tcx > for FindClosureArg < 'tcx > { type NestedFilter = rustc_middle :: hir :: nested_filter :: All ; fn maybe_tcx (& mut self) -> Self :: MaybeTyCtxt { self . tcx } fn visit_expr (& mut self , ex : & 'tcx hir :: Expr < 'tcx >) { if let hir :: ExprKind :: Call (rcvr , args) = ex . kind { self . calls . push ((rcvr , args)) ; } hir :: intravisit :: walk_expr (self , ex) ; } }
    };
}

impl_200!();