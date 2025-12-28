macro_rules! deps {
    () => {
        CanAccessMutGlobal!();
    };
}

macro_rules! eval_to_valtree {
    () => {
        deps!();
        # [doc = " Evaluates a constant and turns it into a type-level constant value."] pub (crate) fn eval_to_valtree < 'tcx > (tcx : TyCtxt < 'tcx > , typing_env : ty :: TypingEnv < 'tcx > , cid : GlobalId < 'tcx > ,) -> EvalToValTreeResult < 'tcx > { debug_assert_eq ! (typing_env . typing_mode , ty :: TypingMode :: PostAnalysis) ; let const_alloc = tcx . eval_to_allocation_raw (typing_env . as_query_input (cid)) ? ; let ecx = mk_eval_cx_to_read_const_val (tcx , DUMMY_SP , typing_env , CanAccessMutGlobal :: No ,) ; let place = ecx . raw_const_to_mplace (const_alloc) . unwrap () ; debug ! (? place) ; let mut num_nodes = 0 ; const_to_valtree_inner (& ecx , & place , & mut num_nodes) }
    };
}

eval_to_valtree!()