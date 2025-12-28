macro_rules! deps {
    () => {
        PredicateObligations!();
        InferCtxt!();
        PredicateObligation!();
        Obligation!();
    };
}

macro_rules! TraitEngine {
    () => {
        deps!();
        pub trait TraitEngine < 'tcx , E : 'tcx > : 'tcx { # [doc = " Requires that `ty` must implement the trait with `def_id` in"] # [doc = " the given environment. This trait must not have any type"] # [doc = " parameters (except for `Self`)."] fn register_bound (& mut self , infcx : & InferCtxt < 'tcx > , param_env : ty :: ParamEnv < 'tcx > , ty : Ty < 'tcx > , def_id : DefId , cause : ObligationCause < 'tcx > ,) { let trait_ref = ty :: TraitRef :: new (infcx . tcx , def_id , [ty]) ; self . register_predicate_obligation (infcx , Obligation { cause , recursion_depth : 0 , param_env , predicate : trait_ref . upcast (infcx . tcx) , } ,) ; } fn register_predicate_obligation (& mut self , infcx : & InferCtxt < 'tcx > , obligation : PredicateObligation < 'tcx > ,) ; fn register_predicate_obligations (& mut self , infcx : & InferCtxt < 'tcx > , obligations : PredicateObligations < 'tcx > ,) { for obligation in obligations { self . register_predicate_obligation (infcx , obligation) ; } } # [must_use] fn select_where_possible (& mut self , infcx : & InferCtxt < 'tcx >) -> Vec < E > ; fn collect_remaining_errors (& mut self , infcx : & InferCtxt < 'tcx >) -> Vec < E > ; # [must_use] fn select_all_or_error (& mut self , infcx : & InferCtxt < 'tcx >) -> Vec < E > { let errors = self . select_where_possible (infcx) ; if ! errors . is_empty () { return errors ; } self . collect_remaining_errors (infcx) } fn has_pending_obligations (& self) -> bool ; fn pending_obligations (& self) -> PredicateObligations < 'tcx > ; # [doc = " Among all pending obligations, collect those are stalled on a inference variable which has"] # [doc = " changed since the last call to `select_where_possible`. Those obligations are marked as"] # [doc = " successful and returned."] fn drain_stalled_obligations_for_coroutines (& mut self , infcx : & InferCtxt < 'tcx > ,) -> PredicateObligations < 'tcx > ; }
    };
}

TraitEngine!()