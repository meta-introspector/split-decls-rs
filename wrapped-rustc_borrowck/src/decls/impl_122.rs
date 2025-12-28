macro_rules! deps {
    () => {
        ConditionVisitor!();
        ReferencedStatementsVisitor!();
    };
}

macro_rules! impl_122 {
    () => {
        deps!();
        impl < 'v , 'tcx > Visitor < 'v > for ConditionVisitor < 'tcx > { fn visit_expr (& mut self , ex : & 'v hir :: Expr < 'v >) { match ex . kind { hir :: ExprKind :: If (cond , body , None) => { if ReferencedStatementsVisitor (& self . spans) . visit_expr (body) . is_break () { self . errors . push ((cond . span , format ! ("if this `if` condition is `false`, {} is not initialized" , self . name ,) ,)) ; self . errors . push ((ex . span . shrink_to_hi () , format ! ("an `else` arm might be missing here, initializing {}" , self . name) ,)) ; } } hir :: ExprKind :: If (cond , body , Some (other)) => { let a = ReferencedStatementsVisitor (& self . spans) . visit_expr (body) . is_break () ; let b = ReferencedStatementsVisitor (& self . spans) . visit_expr (other) . is_break () ; match (a , b) { (true , true) | (false , false) => { } (true , false) => { if other . span . is_desugaring (DesugaringKind :: WhileLoop) { self . errors . push ((cond . span , format ! ("if this condition isn't met and the `while` loop runs 0 \
                                     times, {} is not initialized" , self . name) ,)) ; } else { self . errors . push ((body . span . shrink_to_hi () . until (other . span) , format ! ("if the `if` condition is `false` and this `else` arm is \
                                     executed, {} is not initialized" , self . name) ,)) ; } } (false , true) => { self . errors . push ((cond . span , format ! ("if this condition is `true`, {} is not initialized" , self . name) ,)) ; } } } hir :: ExprKind :: Match (e , arms , loop_desugar) => { let results : Vec < bool > = arms . iter () . map (| arm | ReferencedStatementsVisitor (& self . spans) . visit_arm (arm) . is_break ()) . collect () ; if results . iter () . any (| x | * x) && ! results . iter () . all (| x | * x) { for (arm , seen) in arms . iter () . zip (results) { if ! seen { if loop_desugar == hir :: MatchSource :: ForLoopDesugar { self . errors . push ((e . span , format ! ("if the `for` loop runs 0 times, {} is not initialized" , self . name) ,)) ; } else if let Some (guard) = & arm . guard { if matches ! (self . tcx . hir_node (arm . body . hir_id) , hir :: Node :: Expr (hir :: Expr { kind : hir :: ExprKind :: Ret (_) , .. })) { continue ; } self . errors . push ((arm . pat . span . to (guard . span) , format ! ("if this pattern and condition are matched, {} is not \
                                         initialized" , self . name) ,)) ; } else { if matches ! (self . tcx . hir_node (arm . body . hir_id) , hir :: Node :: Expr (hir :: Expr { kind : hir :: ExprKind :: Ret (_) , .. })) { continue ; } self . errors . push ((arm . pat . span , format ! ("if this pattern is matched, {} is not initialized" , self . name) ,)) ; } } } } } _ => { } } walk_expr (self , ex) ; } }
    };
}

impl_122!();