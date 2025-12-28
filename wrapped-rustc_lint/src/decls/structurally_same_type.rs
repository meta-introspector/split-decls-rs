macro_rules! structurally_same_type {
    () => {
        # [doc = " Checks whether two types are structurally the same enough that the declarations shouldn't"] # [doc = " clash. We need this so we don't emit a lint when two modules both declare an extern struct,"] # [doc = " with the same members (as the declarations shouldn't clash)."] fn structurally_same_type < 'tcx > (tcx : TyCtxt < 'tcx > , typing_env : ty :: TypingEnv < 'tcx > , a : Ty < 'tcx > , b : Ty < 'tcx > ,) -> bool { let mut seen_types = UnordSet :: default () ; let result = structurally_same_type_impl (& mut seen_types , tcx , typing_env , a , b) ; if cfg ! (debug_assertions) && result { let a_layout = tcx . layout_of (typing_env . as_query_input (a)) . unwrap () ; let b_layout = tcx . layout_of (typing_env . as_query_input (b)) . unwrap () ; assert_eq ! (a_layout . backend_repr , b_layout . backend_repr) ; assert_eq ! (a_layout . size , b_layout . size) ; assert_eq ! (a_layout . align , b_layout . align) ; } result }
    };
}

structurally_same_type!();