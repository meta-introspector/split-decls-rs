macro_rules! deps {
    () => {
        BreakFinder!();
    };
}

macro_rules! impl_120 {
    () => {
        deps!();
        impl < 'hir > Visitor < 'hir > for BreakFinder { fn visit_expr (& mut self , ex : & 'hir hir :: Expr < 'hir >) { match ex . kind { hir :: ExprKind :: Break (destination , _) => { self . found_breaks . push ((destination , ex . span)) ; } hir :: ExprKind :: Continue (destination) => { self . found_continues . push ((destination , ex . span)) ; } _ => { } } hir :: intravisit :: walk_expr (self , ex) ; } }
    };
}

impl_120!()