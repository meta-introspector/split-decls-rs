macro_rules! deps {
    () => {
        FindPathCtx!();
        ModuleId!();
        Choice!();
        ItemInNs!();
    };
}

macro_rules! calculate_best_path {
    () => {
        deps!();
        # [tracing :: instrument (skip_all)] fn calculate_best_path (ctx : & FindPathCtx < '_ > , visited_modules : & mut FxHashSet < (ItemInNs , ModuleId) > , item : ItemInNs , max_len : usize , best_choice : & mut Option < Choice > ,) { let fuel = ctx . fuel . get () ; if fuel == 0 { tracing :: warn ! ("ran out of fuel while searching for a path for item {item:?} of krate {:?} from krate {:?}" , item . krate (ctx . db) , ctx . from . krate ()) ; return ; } ctx . fuel . set (fuel - 1) ; if item . krate (ctx . db) == Some (ctx . from . krate) { calculate_best_path_local (ctx , visited_modules , item , max_len , best_choice) } else if ctx . is_std_item { find_in_sysroot (ctx , visited_modules , item , max_len , best_choice) } else { ctx . from . krate . data (ctx . db) . dependencies . iter () . for_each (| dep | { find_in_dep (ctx , visited_modules , item , max_len , best_choice , dep . crate_id) }) ; } }
    };
}

calculate_best_path!()