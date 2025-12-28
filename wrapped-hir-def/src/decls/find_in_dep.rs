macro_rules! deps {
    () => {
        ItemInNs!();
        ModuleId!();
        Choice!();
        FindPathCtx!();
    };
}

macro_rules! find_in_dep {
    () => {
        deps!();
        fn find_in_dep (ctx : & FindPathCtx < '_ > , visited_modules : & mut FxHashSet < (ItemInNs , ModuleId) > , item : ItemInNs , max_len : usize , best_choice : & mut Option < Choice > , dep : Crate ,) { let import_map = ctx . db . import_map (dep) ; let Some (import_info_for) = import_map . import_info_for (item) else { return ; } ; for info in import_info_for { if info . is_doc_hidden { continue ; } let choice = find_path_for_module (ctx , visited_modules , info . container , true , best_choice . as_ref () . map_or (max_len , | it | it . path . len ()) - 1 ,) ; let Some (mut choice) = choice else { continue ; } ; cov_mark :: hit ! (partially_imported) ; if info . is_unstable { if ! ctx . cfg . allow_unstable { continue ; } choice . stability = Unstable ; } Choice :: try_select (best_choice , choice , ctx . cfg . prefer_prelude , info . name . clone ()) ; } }
    };
}

find_in_dep!();