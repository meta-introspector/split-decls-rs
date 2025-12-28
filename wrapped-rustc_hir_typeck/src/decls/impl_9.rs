macro_rules! deps {
    () => {
        DeferredCallResolution!();
        FnCtxt!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl < 'a , 'tcx > DeferredCallResolution < 'tcx > { pub (crate) fn resolve (self , fcx : & FnCtxt < 'a , 'tcx >) { debug ! ("DeferredCallResolution::resolve() {:?}" , self) ; assert ! (fcx . closure_kind (self . closure_ty) . is_some ()) ; match fcx . try_overloaded_call_traits (self . call_expr , self . closure_ty , None) { Some ((autoref , method_callee)) => { let method_sig = method_callee . sig ; debug ! ("attempt_resolution: method_callee={:?}" , method_callee) ; for (method_arg_ty , self_arg_ty) in iter :: zip (method_sig . inputs () . iter () . skip (1) , self . fn_sig . inputs ()) { fcx . demand_eqtype (self . call_expr . span , * self_arg_ty , * method_arg_ty) ; } fcx . demand_eqtype (self . call_expr . span , method_sig . output () , self . fn_sig . output ()) ; let mut adjustments = self . adjustments ; adjustments . extend (autoref) ; fcx . apply_adjustments (self . callee_expr , adjustments) ; fcx . write_method_call_and_enforce_effects (self . call_expr . hir_id , self . call_expr . span , method_callee ,) ; } None => { span_bug ! (self . call_expr . span , "Expected to find a suitable `Fn`/`FnMut`/`FnOnce` implementation for `{}`" , self . closure_ty) } } } }
    };
}

impl_9!()