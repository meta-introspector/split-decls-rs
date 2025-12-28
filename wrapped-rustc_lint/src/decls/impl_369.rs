macro_rules! deps {
    () => {
        LifetimeInfoMap!();
        LifetimeInfoCollector!();
    };
}

macro_rules! impl_369 {
    () => {
        deps!();
        impl < 'a , 'tcx > LifetimeInfoCollector < 'a , 'tcx > { fn collect (ty : & 'tcx hir :: Ty < 'tcx > , map : & 'a mut LifetimeInfoMap < 'tcx >) { let mut this = Self { type_span : ty . span , referenced_type_span : None , map } ; intravisit :: walk_unambig_ty (& mut this , ty) ; } }
    };
}

impl_369!();