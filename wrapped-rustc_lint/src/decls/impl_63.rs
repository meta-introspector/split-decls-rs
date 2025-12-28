macro_rules! deps {
    () => {
        LateContext!();
    };
}

macro_rules! impl_63 {
    () => {
        deps!();
        impl < 'tcx > LateLintPass < 'tcx > for UnreachablePub { fn check_item (& mut self , cx : & LateContext < '_ > , item : & hir :: Item < '_ >) { if let hir :: ItemKind :: Use (_ , hir :: UseKind :: ListStem) = & item . kind { return ; } self . perform_lint (cx , "item" , item . owner_id . def_id , item . vis_span , true) ; } fn check_foreign_item (& mut self , cx : & LateContext < '_ > , foreign_item : & hir :: ForeignItem < 'tcx >) { self . perform_lint (cx , "item" , foreign_item . owner_id . def_id , foreign_item . vis_span , true) ; } fn check_field_def (& mut self , _cx : & LateContext < '_ > , _field : & hir :: FieldDef < '_ >) { } fn check_impl_item (& mut self , cx : & LateContext < '_ > , impl_item : & hir :: ImplItem < '_ >) { if let ImplItemImplKind :: Inherent { vis_span } = impl_item . impl_kind { self . perform_lint (cx , "item" , impl_item . owner_id . def_id , vis_span , false) ; } } }
    };
}

impl_63!();