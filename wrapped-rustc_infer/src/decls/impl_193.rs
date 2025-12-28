macro_rules! deps {
    () => {
        InferenceFudger!();
    };
}

macro_rules! impl_193 {
    () => {
        deps!();
        impl < 'a , 'tcx > TypeFolder < TyCtxt < 'tcx > > for InferenceFudger < 'a , 'tcx > { fn cx (& self) -> TyCtxt < 'tcx > { self . infcx . tcx } fn fold_ty (& mut self , ty : Ty < 'tcx >) -> Ty < 'tcx > { if let & ty :: Infer (infer_ty) = ty . kind () { match infer_ty { ty :: TyVar (vid) => { if self . snapshot_vars . type_vars . 0 . contains (& vid) { let idx = vid . as_usize () - self . snapshot_vars . type_vars . 0 . start . as_usize () ; let origin = self . snapshot_vars . type_vars . 1 [idx] ; self . infcx . next_ty_var_with_origin (origin) } else { debug_assert ! (self . infcx . inner . borrow_mut () . type_variables () . probe (vid) . is_unknown ()) ; ty } } ty :: IntVar (vid) => { if self . snapshot_vars . int_vars . contains (& vid) { self . infcx . next_int_var () } else { ty } } ty :: FloatVar (vid) => { if self . snapshot_vars . float_vars . contains (& vid) { self . infcx . next_float_var () } else { ty } } ty :: FreshTy (_) | ty :: FreshIntTy (_) | ty :: FreshFloatTy (_) => { unreachable ! ("unexpected fresh infcx var") } } } else if ty . has_infer () { ty . super_fold_with (self) } else { ty } } fn fold_region (& mut self , r : ty :: Region < 'tcx >) -> ty :: Region < 'tcx > { if let ty :: ReVar (vid) = r . kind () { if self . snapshot_vars . region_vars . 0 . contains (& vid) { let idx = vid . index () - self . snapshot_vars . region_vars . 0 . start . index () ; let origin = self . snapshot_vars . region_vars . 1 [idx] ; self . infcx . next_region_var (origin) } else { r } } else { r } } fn fold_const (& mut self , ct : ty :: Const < 'tcx >) -> ty :: Const < 'tcx > { if let ty :: ConstKind :: Infer (infer_ct) = ct . kind () { match infer_ct { ty :: InferConst :: Var (vid) => { if self . snapshot_vars . const_vars . 0 . contains (& vid) { let idx = vid . index () - self . snapshot_vars . const_vars . 0 . start . index () ; let origin = self . snapshot_vars . const_vars . 1 [idx] ; self . infcx . next_const_var_with_origin (origin) } else { ct } } ty :: InferConst :: Fresh (_) => { unreachable ! ("unexpected fresh infcx var") } } } else if ct . has_infer () { ct . super_fold_with (self) } else { ct } } }
    };
}

impl_193!()