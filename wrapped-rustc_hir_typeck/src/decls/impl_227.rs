macro_rules! deps {
    () => {
        GatherLocalsVisitor!();
    };
}

macro_rules! impl_227 {
    () => {
        deps!();
        impl < 'a , 'tcx > Visitor < 'tcx > for GatherLocalsVisitor < 'a , 'tcx > { fn visit_local (& mut self , local : & 'tcx hir :: LetStmt < 'tcx >) { self . declare (local . into ()) ; intravisit :: walk_local (self , local) } fn visit_expr (& mut self , expr : & 'tcx hir :: Expr < 'tcx >) { if let hir :: ExprKind :: Let (let_expr) = expr . kind { self . declare ((let_expr , expr . hir_id) . into ()) ; } intravisit :: walk_expr (self , expr) } fn visit_pat (& mut self , p : & 'tcx hir :: Pat < 'tcx >) { if let PatKind :: Binding (_ , _ , ident , _) = p . kind { let var_ty = self . assign (p . span , p . hir_id , None) ; if let Some ((ty_span , hir_id)) = self . outermost_fn_param_pat { if ! self . fcx . tcx . features () . unsized_fn_params () { self . fcx . require_type_is_sized (var_ty , ty_span , ObligationCauseCode :: SizedArgumentType (if ty_span == ident . span && self . fcx . tcx . is_closure_like (self . fcx . body_id . into ()) { None } else { Some (hir_id) } ,) ,) ; } } else { self . fcx . require_type_is_sized (var_ty , p . span , ObligationCauseCode :: VariableType (p . hir_id) ,) ; } debug ! ("pattern binding {} is assigned to {} with type {:?}" , ident , self . fcx . ty_to_string (* self . fcx . locals . borrow () . get (& p . hir_id) . unwrap ()) , var_ty) ; } let old_outermost_fn_param_pat = self . outermost_fn_param_pat . take () ; if let PatKind :: Guard (subpat , _) = p . kind { self . visit_pat (subpat) ; } else { intravisit :: walk_pat (self , p) ; } self . outermost_fn_param_pat = old_outermost_fn_param_pat ; } fn visit_fn (& mut self , _ : intravisit :: FnKind < 'tcx > , _ : & 'tcx hir :: FnDecl < 'tcx > , _ : hir :: BodyId , _ : Span , _ : LocalDefId ,) { } }
    };
}

impl_227!()