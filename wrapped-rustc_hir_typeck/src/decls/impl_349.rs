macro_rules! deps {
    () => {
        InferBorrowKindVisitor!();
    };
}

macro_rules! impl_349 {
    () => {
        deps!();
        impl < 'a , 'tcx > Visitor < 'tcx > for InferBorrowKindVisitor < 'a , 'tcx > { fn visit_expr (& mut self , expr : & 'tcx hir :: Expr < 'tcx >) { match expr . kind { hir :: ExprKind :: Closure (& hir :: Closure { capture_clause , body : body_id , .. }) => { let body = self . fcx . tcx . hir_body (body_id) ; self . visit_body (body) ; self . fcx . analyze_closure (expr . hir_id , expr . span , body_id , body , capture_clause) ; } _ => { } } intravisit :: walk_expr (self , expr) ; } fn visit_inline_const (& mut self , c : & 'tcx hir :: ConstBlock) { let body = self . fcx . tcx . hir_body (c . body) ; self . visit_body (body) ; } }
    };
}

impl_349!()