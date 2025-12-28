macro_rules! deps {
    () => {
        ModuleDefId!();
        FindPathCtx!();
        ItemInNs!();
        ModuleId!();
        PrefixKind!();
        Choice!();
    };
}

macro_rules! find_path_for_module {
    () => {
        deps!();
        # [tracing :: instrument (skip_all)] fn find_path_for_module (ctx : & FindPathCtx < '_ > , visited_modules : & mut FxHashSet < (ItemInNs , ModuleId) > , module_id : ModuleId , maybe_extern : bool , max_len : usize ,) -> Option < Choice > { if max_len == 0 { return None ; } if let Some (crate_root) = module_id . as_crate_root () { if ! maybe_extern || crate_root == ctx . from . derive_crate_root () { return Some (Choice { path : ModPath :: from_segments (PathKind :: Crate , None) , path_text_len : 5 , stability : Stable , prefer_due_to_prelude : false , }) ; } let root_local_def_map = ctx . from . derive_crate_root () . local_def_map (ctx . db) . 1 ; for (name , (def_id , _extern_crate)) in root_local_def_map . extern_prelude () . rev () { if crate_root != def_id { continue ; } let name_already_occupied_in_type_ns = ctx . from_def_map . with_ancestor_maps (ctx . db , ctx . from . local_id , & mut | def_map , local_id | { def_map [local_id] . scope . type_ (name) . filter (| & (id , _) | id != ModuleDefId :: ModuleId (def_id . into ())) }) . is_some () ; let kind = if name_already_occupied_in_type_ns { cov_mark :: hit ! (ambiguous_crate_start) ; PathKind :: Abs } else if ctx . cfg . prefer_absolute { PathKind :: Abs } else { PathKind :: Plain } ; return Some (Choice :: new (ctx . cfg . prefer_prelude , kind , name . clone () , Stable)) ; } } let may_be_in_scope = match ctx . prefix { PrefixKind :: Plain | PrefixKind :: BySelf => true , PrefixKind :: ByCrate => ctx . from . is_crate_root () , } ; if may_be_in_scope { let scope_name = find_in_scope (ctx . db , ctx . from_def_map , ctx . from , ItemInNs :: Types (module_id . into ()) , ctx . ignore_local_imports ,) ; if let Some (scope_name) = scope_name { return Some (Choice :: new (ctx . cfg . prefer_prelude , ctx . prefix . path_kind () , scope_name , Stable ,)) ; } } if let Some (kind) = is_kw_kind_relative_to_from (ctx . from_def_map , module_id , ctx . from) && (ctx . prefix != PrefixKind :: ByCrate || kind == PathKind :: Crate) { return Some (Choice { path : ModPath :: from_segments (kind , None) , path_text_len : path_kind_len (kind) , stability : Stable , prefer_due_to_prelude : false , }) ; } let item = ItemInNs :: Types (module_id . into ()) ; if let Some (choice) = find_in_prelude (ctx . db , ctx . from_def_map , item , ctx . from) { return Some (choice) ; } let mut best_choice = None ; if maybe_extern { calculate_best_path (ctx , visited_modules , item , max_len , & mut best_choice) ; } else { calculate_best_path_local (ctx , visited_modules , item , max_len , & mut best_choice) ; } best_choice }
    };
}

find_path_for_module!();