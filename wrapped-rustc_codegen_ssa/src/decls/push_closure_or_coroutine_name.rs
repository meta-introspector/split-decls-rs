macro_rules! push_closure_or_coroutine_name {
    () => {
        fn push_closure_or_coroutine_name < 'tcx > (tcx : TyCtxt < 'tcx > , def_id : DefId , args : GenericArgsRef < 'tcx > , qualified : bool , output : & mut String , visited : & mut FxHashSet < Ty < 'tcx > > ,) { let def_key = tcx . def_key (def_id) ; let coroutine_kind = tcx . coroutine_kind (def_id) ; if qualified { let parent_def_id = DefId { index : def_key . parent . unwrap () , .. def_id } ; push_item_name (tcx , parent_def_id , true , output) ; output . push_str ("::") ; } let mut label = String :: with_capacity (20) ; write ! (& mut label , "{}_env" , coroutine_kind_label (coroutine_kind)) . unwrap () ; push_disambiguated_special_name (& label , def_key . disambiguated_data . disambiguator , cpp_like_debuginfo (tcx) , output ,) ; let enclosing_fn_def_id = tcx . typeck_root_def_id (def_id) ; let generics = tcx . generics_of (enclosing_fn_def_id) ; let args = args . truncate_to (tcx , generics) ; push_generic_params_internal (tcx , args , output , visited) ; }
    };
}

push_closure_or_coroutine_name!();