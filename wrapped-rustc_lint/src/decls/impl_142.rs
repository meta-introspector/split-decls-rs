macro_rules! deps {
    () => {
        DanglingPointerReturnSearcher!();
    };
}

macro_rules! impl_142 {
    () => {
        deps!();
        impl < 'tcx > Visitor < 'tcx > for DanglingPointerReturnSearcher < '_ , 'tcx > { fn visit_expr (& mut self , expr : & 'tcx Expr < 'tcx >) -> Self :: Result { if let ExprKind :: Ret (Some (expr)) = expr . kind { lint_addr_of_local (self . cx , self . dcx , expr) ; } walk_expr (self , expr) } }
    };
}

impl_142!()