macro_rules! augment_param_env {
    () => {
        # [doc = " Add a new set of predicates to the caller_bounds of an existing param_env."] fn augment_param_env < 'tcx > (tcx : TyCtxt < 'tcx > , param_env : ty :: ParamEnv < 'tcx > , new_predicates : Option < & FxIndexSet < ty :: Clause < 'tcx > > > ,) -> ty :: ParamEnv < 'tcx > { let Some (new_predicates) = new_predicates else { return param_env ; } ; if new_predicates . is_empty () { return param_env ; } let bounds = tcx . mk_clauses_from_iter (param_env . caller_bounds () . iter () . chain (new_predicates . iter () . cloned ()) ,) ; ty :: ParamEnv :: new (bounds) }
    };
}

augment_param_env!();