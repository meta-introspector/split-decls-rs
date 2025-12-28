macro_rules! deps {
    () => {
        ToArgRegionsFolder!();
    };
}

macro_rules! impl_347 {
    () => {
        deps!();
        impl < 'tcx > FallibleTypeFolder < TyCtxt < 'tcx > > for ToArgRegionsFolder < '_ , 'tcx > { type Error = RegionVid ; fn cx (& self) -> TyCtxt < 'tcx > { self . rcx . infcx . tcx } fn try_fold_region (& mut self , r : Region < 'tcx >) -> Result < Region < 'tcx > , RegionVid > { match r . kind () { ty :: ReBound (_ , _) => Ok (r) , _ => { let r = r . as_var () ; if let Some (arg_region) = self . arg_regions . iter () . copied () . find (| & arg_vid | self . rcx . eval_equal (r , arg_vid)) . and_then (| r | nll_var_to_universal_region (self . rcx , r)) { Ok (arg_region) } else if self . erase_unknown_regions { Ok (self . cx () . lifetimes . re_erased) } else { Err (r) } } } } fn try_fold_ty (& mut self , ty : Ty < 'tcx >) -> Result < Ty < 'tcx > , RegionVid > { if ! ty . flags () . intersects (ty :: TypeFlags :: HAS_FREE_REGIONS) { return Ok (ty) ; } let tcx = self . cx () ; Ok (match * ty . kind () { ty :: Closure (def_id , args) => { Ty :: new_closure (tcx , def_id , self . fold_closure_args (def_id , args) ?) } ty :: CoroutineClosure (def_id , args) => { Ty :: new_coroutine_closure (tcx , def_id , self . fold_closure_args (def_id , args) ?) } ty :: Coroutine (def_id , args) => { Ty :: new_coroutine (tcx , def_id , self . fold_closure_args (def_id , args) ?) } ty :: Alias (kind , ty :: AliasTy { def_id , args , .. }) if let Some (variances) = tcx . opt_alias_variances (kind , def_id) => { let args = tcx . mk_args_from_iter (std :: iter :: zip (variances , args . iter ()) . map (| (& v , s) | { if v == ty :: Bivariant { Ok (self . fold_non_member_arg (s)) } else { s . try_fold_with (self) } } ,)) ? ; ty :: AliasTy :: new_from_args (tcx , def_id , args) . to_ty (tcx) } _ => ty . try_super_fold_with (self) ? , }) } }
    };
}

impl_347!();