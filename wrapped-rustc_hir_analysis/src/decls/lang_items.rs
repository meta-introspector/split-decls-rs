macro_rules! lang_items {
    () => {
        fn lang_items (tcx : TyCtxt < '_ >) -> Vec < (LocalDefId , Vec < ty :: Variance >) > { let lang_items = tcx . lang_items () ; let all = [(lang_items . phantom_data () , vec ! [ty :: Covariant]) , (lang_items . unsafe_cell_type () , vec ! [ty :: Invariant]) ,] ; all . into_iter () . filter_map (| (d , v) | { let def_id = d ? . as_local () ? ; Some ((def_id , v)) }) . collect () }
    };
}

lang_items!()