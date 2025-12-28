macro_rules! deps {
    () => {
        OpportunisticVarResolver!();
    };
}

macro_rules! impl_179 {
    () => {
        deps!();
        impl < 'a , 'tcx > TypeFolder < TyCtxt < 'tcx > > for OpportunisticVarResolver < 'a , 'tcx > { fn cx (& self) -> TyCtxt < 'tcx > { self . infcx . tcx } # [inline] fn fold_ty (& mut self , t : Ty < 'tcx >) -> Ty < 'tcx > { if ! t . has_non_region_infer () { t } else if let Some (& ty) = self . cache . get (& t) { ty } else { let shallow = self . infcx . shallow_resolve (t) ; let res = shallow . super_fold_with (self) ; assert ! (self . cache . insert (t , res)) ; res } } fn fold_const (& mut self , ct : Const < 'tcx >) -> Const < 'tcx > { if ! ct . has_non_region_infer () { ct } else { let ct = self . infcx . shallow_resolve_const (ct) ; ct . super_fold_with (self) } } fn fold_predicate (& mut self , p : ty :: Predicate < 'tcx >) -> ty :: Predicate < 'tcx > { if ! p . has_non_region_infer () { p } else { p . super_fold_with (self) } } fn fold_clauses (& mut self , c : ty :: Clauses < 'tcx >) -> ty :: Clauses < 'tcx > { if ! c . has_non_region_infer () { c } else { c . super_fold_with (self) } } }
    };
}

impl_179!();