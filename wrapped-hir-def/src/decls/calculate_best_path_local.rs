macro_rules! deps {
    () => {
        ModuleId!();
        FindPathCtx!();
        ItemInNs!();
        Choice!();
    };
}

macro_rules! calculate_best_path_local {
    () => {
        deps!();
        fn calculate_best_path_local (ctx : & FindPathCtx < '_ > , visited_modules : & mut FxHashSet < (ItemInNs , ModuleId) > , item : ItemInNs , max_len : usize , best_choice : & mut Option < Choice > ,) { find_local_import_locations (ctx . db , item , ctx . from , ctx . from_def_map , visited_modules , | visited_modules , name , module_id | { if let Some (choice) = find_path_for_module (ctx , visited_modules , module_id , false , best_choice . as_ref () . map_or (max_len , | it | it . path . len ()) - 1 ,) { Choice :: try_select (best_choice , choice , ctx . cfg . prefer_prelude , name . clone ()) ; } } ,) ; }
    };
}

calculate_best_path_local!();