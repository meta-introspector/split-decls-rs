macro_rules! push_unqualified_item_name {
    () => {
        fn push_unqualified_item_name (tcx : TyCtxt < '_ > , def_id : DefId , disambiguated_data : DisambiguatedDefPathData , output : & mut String ,) { match disambiguated_data . data { DefPathData :: CrateRoot => { output . push_str (tcx . crate_name (def_id . krate) . as_str ()) ; } DefPathData :: Closure => { let label = coroutine_kind_label (tcx . coroutine_kind (def_id)) ; push_disambiguated_special_name (label , disambiguated_data . disambiguator , cpp_like_debuginfo (tcx) , output ,) ; } _ => match disambiguated_data . data . name () { DefPathDataName :: Named (name) => { output . push_str (name . as_str ()) ; } DefPathDataName :: Anon { namespace } => { push_disambiguated_special_name (namespace . as_str () , disambiguated_data . disambiguator , cpp_like_debuginfo (tcx) , output ,) ; } } , } ; }
    };
}

push_unqualified_item_name!()