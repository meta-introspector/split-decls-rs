macro_rules! deps {
    () => {
        Info!();
        LifetimeInfoCollector!();
    };
}

macro_rules! impl_370 {
    () => {
        deps!();
        impl < 'a , 'tcx > Visitor < 'tcx > for LifetimeInfoCollector < 'a , 'tcx > { # [instrument (skip (self))] fn visit_lifetime (& mut self , lifetime : & 'tcx hir :: Lifetime) { let type_span = self . type_span ; let referenced_type_span = self . referenced_type_span ; let info = Info { type_span , referenced_type_span , lifetime } ; self . map . entry (& lifetime . kind) . or_default () . push (info) ; } # [instrument (skip (self))] fn visit_ty (& mut self , ty : & 'tcx hir :: Ty < 'tcx , hir :: AmbigArg >) -> Self :: Result { let old_type_span = self . type_span ; let old_referenced_type_span = self . referenced_type_span ; self . type_span = ty . span ; if let hir :: TyKind :: Ref (_ , ty) = ty . kind { self . referenced_type_span = Some (ty . ty . span) ; } intravisit :: walk_ty (self , ty) ; self . type_span = old_type_span ; self . referenced_type_span = old_referenced_type_span ; } }
    };
}

impl_370!()