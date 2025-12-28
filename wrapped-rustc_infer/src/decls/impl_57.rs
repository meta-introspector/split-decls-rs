macro_rules! deps {
    () => {
        TypeFreshener!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        impl < 'a , 'tcx > TypeFolder < TyCtxt < 'tcx > > for TypeFreshener < 'a , 'tcx > { fn cx (& self) -> TyCtxt < 'tcx > { self . infcx . tcx } fn fold_region (& mut self , r : ty :: Region < 'tcx >) -> ty :: Region < 'tcx > { match r . kind () { ty :: ReBound (..) => r , ty :: ReError (_) => r , ty :: ReEarlyParam (..) | ty :: ReLateParam (_) | ty :: ReVar (_) | ty :: RePlaceholder (..) | ty :: ReStatic | ty :: ReErased => self . cx () . lifetimes . re_erased , } } # [inline] fn fold_ty (& mut self , t : Ty < 'tcx >) -> Ty < 'tcx > { if ! t . has_infer () && ! t . has_erasable_regions () { t } else { match * t . kind () { ty :: Infer (v) => self . fold_infer_ty (v) . unwrap_or (t) , # [cfg (debug_assertions)] ty :: Placeholder (..) | ty :: Bound (..) => bug ! ("unexpected type {:?}" , t) , _ => t . super_fold_with (self) , } } } fn fold_const (& mut self , ct : ty :: Const < 'tcx >) -> ty :: Const < 'tcx > { match ct . kind () { ty :: ConstKind :: Infer (ty :: InferConst :: Var (v)) => { let mut inner = self . infcx . inner . borrow_mut () ; let input = inner . const_unification_table () . probe_value (v) . known () . ok_or_else (| | { ty :: InferConst :: Var (inner . const_unification_table () . find (v) . vid) }) ; drop (inner) ; self . freshen_const (input , ty :: InferConst :: Fresh) } ty :: ConstKind :: Infer (ty :: InferConst :: Fresh (i)) => { if i >= self . const_freshen_count { bug ! ("Encountered a freshend const with id {} \
                            but our counter is only at {}" , i , self . const_freshen_count ,) ; } ct } ty :: ConstKind :: Bound (..) | ty :: ConstKind :: Placeholder (_) => { bug ! ("unexpected const {:?}" , ct) } ty :: ConstKind :: Param (_) | ty :: ConstKind :: Value (_) | ty :: ConstKind :: Unevaluated (..) | ty :: ConstKind :: Expr (..) | ty :: ConstKind :: Error (_) => ct . super_fold_with (self) , } } }
    };
}

impl_57!();