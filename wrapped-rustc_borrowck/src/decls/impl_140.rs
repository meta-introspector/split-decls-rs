macro_rules! deps {
    () => {
        BindingFinder!();
    };
}

macro_rules! impl_140 {
    () => {
        deps!();
        impl < 'tcx > Visitor < 'tcx > for BindingFinder { type Result = ControlFlow < hir :: HirId > ; fn visit_stmt (& mut self , s : & 'tcx hir :: Stmt < 'tcx >) -> Self :: Result { if let hir :: StmtKind :: Let (local) = s . kind && local . pat . span == self . span { ControlFlow :: Break (local . hir_id) } else { hir :: intravisit :: walk_stmt (self , s) } } fn visit_param (& mut self , param : & 'tcx hir :: Param < 'tcx >) -> Self :: Result { if let hir :: Pat { kind : hir :: PatKind :: Ref (_ , _) , span , .. } = param . pat && * span == self . span { ControlFlow :: Break (param . hir_id) } else { ControlFlow :: Continue (()) } } }
    };
}

impl_140!();