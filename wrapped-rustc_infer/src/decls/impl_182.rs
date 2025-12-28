macro_rules! deps {
    () => {
        OpportunisticRegionResolver!();
    };
}

macro_rules! impl_182 {
    () => {
        deps!();
        impl < 'a , 'tcx > TypeFolder < TyCtxt < 'tcx > > for OpportunisticRegionResolver < 'a , 'tcx > { fn cx (& self) -> TyCtxt < 'tcx > { self . infcx . tcx } fn fold_ty (& mut self , t : Ty < 'tcx >) -> Ty < 'tcx > { if ! t . has_infer_regions () { t } else { t . super_fold_with (self) } } fn fold_region (& mut self , r : ty :: Region < 'tcx >) -> ty :: Region < 'tcx > { match r . kind () { ty :: ReVar (vid) => self . infcx . inner . borrow_mut () . unwrap_region_constraints () . opportunistic_resolve_var (TypeFolder :: cx (self) , vid) , _ => r , } } fn fold_const (& mut self , ct : ty :: Const < 'tcx >) -> ty :: Const < 'tcx > { if ! ct . has_infer_regions () { ct } else { ct . super_fold_with (self) } } }
    };
}

impl_182!()