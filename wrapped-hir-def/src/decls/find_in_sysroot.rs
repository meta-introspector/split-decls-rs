macro_rules! deps {
    () => {
        FindPathCtx!();
        Choice!();
        ItemInNs!();
        ModuleId!();
    };
}

macro_rules! find_in_sysroot {
    () => {
        deps!();
        fn find_in_sysroot (ctx : & FindPathCtx < '_ > , visited_modules : & mut FxHashSet < (ItemInNs , ModuleId) > , item : ItemInNs , max_len : usize , best_choice : & mut Option < Choice > ,) { let dependencies = & ctx . from . krate . data (ctx . db) . dependencies ; let mut search = | lang , best_choice : & mut _ | { if let Some (dep) = dependencies . iter () . filter (| it | it . is_sysroot ()) . find (| dep | { match dep . crate_id . data (ctx . db) . origin { CrateOrigin :: Lang (l) => l == lang , _ => false , } }) { find_in_dep (ctx , visited_modules , item , max_len , best_choice , dep . crate_id) ; } } ; if ctx . cfg . prefer_no_std { search (LangCrateOrigin :: Core , best_choice) ; if matches ! (best_choice , Some (Choice { stability : Stable , .. })) { return ; } search (LangCrateOrigin :: Std , best_choice) ; if matches ! (best_choice , Some (Choice { stability : Stable , .. })) { return ; } } else { search (LangCrateOrigin :: Std , best_choice) ; if matches ! (best_choice , Some (Choice { stability : Stable , .. })) { return ; } search (LangCrateOrigin :: Core , best_choice) ; if matches ! (best_choice , Some (Choice { stability : Stable , .. })) { return ; } } dependencies . iter () . filter (| it | it . is_sysroot ()) . chain (dependencies . iter () . filter (| it | ! it . is_sysroot ())) . for_each (| dep | { find_in_dep (ctx , visited_modules , item , max_len , best_choice , dep . crate_id) ; }) ; }
    };
}

find_in_sysroot!();