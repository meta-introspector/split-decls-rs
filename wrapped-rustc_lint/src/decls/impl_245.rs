macro_rules! deps {
    () => {
        FindSignificantDropper!();
    };
}

macro_rules! impl_245 {
    () => {
        deps!();
        impl < 'tcx > Visitor < 'tcx > for FindSignificantDropper < '_ , 'tcx > { type Result = ControlFlow < (Span , SmallVec < [Ty < 'tcx > ; 4] >) > ; fn visit_block (& mut self , b : & 'tcx hir :: Block < 'tcx >) -> Self :: Result { if let Some (expr) = b . expr { self . visit_expr (expr) } else { ControlFlow :: Continue (()) } } fn visit_expr (& mut self , expr : & 'tcx hir :: Expr < 'tcx >) -> Self :: Result { for adj in self . cx . typeck_results () . expr_adjustments (expr) { match adj . kind { Adjust :: Deref (_) => break , Adjust :: Borrow (_) => { self . check_promoted_temp_with_drop (expr) ? ; } _ => { } } } match expr . kind { hir :: ExprKind :: AddrOf (_ , _ , expr) => { self . check_promoted_temp_with_drop (expr) ? ; intravisit :: walk_expr (self , expr) } hir :: ExprKind :: Index (expr , _ , _) | hir :: ExprKind :: Field (expr , _) => { self . check_promoted_temp_with_drop (expr) ? ; intravisit :: walk_expr (self , expr) } hir :: ExprKind :: If (..) => ControlFlow :: Continue (()) , hir :: ExprKind :: Match (scrut , _ , _) => self . visit_expr (scrut) , hir :: ExprKind :: DropTemps (_) => ControlFlow :: Continue (()) , _ => intravisit :: walk_expr (self , expr) , } } }
    };
}

impl_245!();