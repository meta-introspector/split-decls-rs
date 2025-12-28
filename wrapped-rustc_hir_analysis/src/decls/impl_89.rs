macro_rules! deps {
    () => {
        LetKind!();
        ScopeResolutionVisitor!();
    };
}

macro_rules! impl_89 {
    () => {
        deps!();
        impl < 'tcx > Visitor < 'tcx > for ScopeResolutionVisitor < 'tcx > { fn visit_block (& mut self , b : & 'tcx Block < 'tcx >) { resolve_block (self , b , false) ; } fn visit_body (& mut self , body : & hir :: Body < 'tcx >) { let body_id = body . id () ; let owner_id = self . tcx . hir_body_owner_def_id (body_id) ; debug ! ("visit_body(id={:?}, span={:?}, body.id={:?}, cx.parent={:?})" , owner_id , self . tcx . sess . source_map () . span_to_diagnostic_string (body . value . span) , body_id , self . cx . parent) ; self . enter_body (body . value . hir_id , | this | { if this . tcx . hir_body_owner_kind (owner_id) . is_fn_or_closure () { this . cx . var_parent = (this . cx . parent , ScopeCompatibility :: FutureCompatible) ; for param in body . params { this . visit_pat (param . pat) ; } resolve_expr (this , body . value , true) ; } else { this . cx . var_parent = (None , ScopeCompatibility :: FutureCompatible) ; this . enter_scope (Scope { local_id : body . value . hir_id . local_id , data : ScopeData :: Destruction , }) ; resolve_local (this , None , Some (body . value) , LetKind :: Regular) ; } }) } fn visit_arm (& mut self , a : & 'tcx Arm < 'tcx >) { resolve_arm (self , a) ; } fn visit_pat (& mut self , p : & 'tcx Pat < 'tcx >) { resolve_pat (self , p) ; } fn visit_stmt (& mut self , s : & 'tcx Stmt < 'tcx >) { resolve_stmt (self , s) ; } fn visit_expr (& mut self , ex : & 'tcx Expr < 'tcx >) { resolve_expr (self , ex , false) ; } fn visit_local (& mut self , l : & 'tcx LetStmt < 'tcx >) { let let_kind = match l . super_ { Some (_) => LetKind :: Super , None => LetKind :: Regular , } ; resolve_local (self , Some (l . pat) , l . init , let_kind) ; } fn visit_inline_const (& mut self , c : & 'tcx hir :: ConstBlock) { let body = self . tcx . hir_body (c . body) ; self . visit_body (body) ; } }
    };
}

impl_89!()