macro_rules! deps {
    () => {
        ModuleId!();
        PrefixKind!();
        ItemInNs!();
        ModuleDefId!();
        FindPathCtx!();
    };
}

macro_rules! find_path_inner {
    () => {
        deps!();
        # [doc = " Attempts to find a path to refer to the given `item` visible from the `from` ModuleId"] fn find_path_inner (ctx : & FindPathCtx < '_ > , item : ItemInNs , max_len : usize) -> Option < ModPath > { if ! ctx . is_std_item && let ItemInNs :: Types (ModuleDefId :: ModuleId (module_id)) = item { return find_path_for_module (ctx , & mut FxHashSet :: default () , module_id , true , max_len) . map (| choice | choice . path) ; } let may_be_in_scope = match ctx . prefix { PrefixKind :: Plain | PrefixKind :: BySelf => true , PrefixKind :: ByCrate => ctx . from . is_crate_root () , } ; if may_be_in_scope { let scope_name = find_in_scope (ctx . db , ctx . from_def_map , ctx . from , item , ctx . ignore_local_imports) ; if let Some (scope_name) = scope_name { return Some (ModPath :: from_segments (ctx . prefix . path_kind () , iter :: once (scope_name))) ; } } if let Some (value) = find_in_prelude (ctx . db , ctx . from_def_map , item , ctx . from) { return Some (value . path) ; } if let Some (ModuleDefId :: EnumVariantId (variant)) = item . as_module_def_id () { let loc = variant . lookup (ctx . db) ; if let Some (mut path) = find_path_inner (ctx , ItemInNs :: Types (loc . parent . into ()) , max_len) { path . push_segment (loc . parent . enum_variants (ctx . db) . variants [loc . index as usize] . 1 . clone () ,) ; return Some (path) ; } } let mut best_choice = None ; calculate_best_path (ctx , & mut FxHashSet :: default () , item , max_len , & mut best_choice) ; best_choice . map (| choice | choice . path) }
    };
}

find_path_inner!()