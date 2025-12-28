macro_rules! deps {
    () => {
        CanonicalInstantiator!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl < 'tcx > TypeFolder < TyCtxt < 'tcx > > for CanonicalInstantiator < 'tcx > { fn cx (& self) -> TyCtxt < 'tcx > { self . tcx } fn fold_binder < T : TypeFoldable < TyCtxt < 'tcx > > > (& mut self , t : ty :: Binder < 'tcx , T > ,) -> ty :: Binder < 'tcx , T > { self . current_index . shift_in (1) ; let t = t . super_fold_with (self) ; self . current_index . shift_out (1) ; t } fn fold_ty (& mut self , t : Ty < 'tcx >) -> Ty < 'tcx > { match * t . kind () { ty :: Bound (debruijn , bound_ty) if debruijn == self . current_index => { self . var_values [bound_ty . var . as_usize ()] . expect_ty () } _ => { if ! t . has_vars_bound_at_or_above (self . current_index) { t } else if let Some (& t) = self . cache . get (& (self . current_index , t)) { t } else { let res = t . super_fold_with (self) ; assert ! (self . cache . insert ((self . current_index , t) , res)) ; res } } } } fn fold_region (& mut self , r : ty :: Region < 'tcx >) -> ty :: Region < 'tcx > { match r . kind () { ty :: ReBound (debruijn , br) if debruijn == self . current_index => { self . var_values [br . var . as_usize ()] . expect_region () } _ => r , } } fn fold_const (& mut self , ct : ty :: Const < 'tcx >) -> ty :: Const < 'tcx > { match ct . kind () { ty :: ConstKind :: Bound (debruijn , bound_const) if debruijn == self . current_index => { self . var_values [bound_const . var . as_usize ()] . expect_const () } _ => ct . super_fold_with (self) , } } fn fold_predicate (& mut self , p : ty :: Predicate < 'tcx >) -> ty :: Predicate < 'tcx > { if p . has_vars_bound_at_or_above (self . current_index) { p . super_fold_with (self) } else { p } } fn fold_clauses (& mut self , c : ty :: Clauses < 'tcx >) -> ty :: Clauses < 'tcx > { if ! c . has_vars_bound_at_or_above (self . current_index) { return c ; } if self . current_index > ty :: INNERMOST { return c . super_fold_with (self) ; } let index = * self . tcx . highest_var_in_clauses_cache . lock () . entry (c) . or_insert_with (| | highest_var_in_clauses (c)) ; let c_args = & self . var_values [..= index] ; if let Some (c) = self . tcx . clauses_cache . lock () . get (& (c , c_args)) { c } else { let folded = c . super_fold_with (self) ; self . tcx . clauses_cache . lock () . insert ((c , c_args) , folded) ; folded } } }
    };
}

impl_40!();