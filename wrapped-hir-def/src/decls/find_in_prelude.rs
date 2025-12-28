macro_rules! deps {
    () => {
        ModuleId!();
        DefDatabase!();
        DefMap!();
        ItemInNs!();
        Choice!();
    };
}

macro_rules! find_in_prelude {
    () => {
        deps!();
        # [doc = " Returns single-segment path (i.e. without any prefix) if `item` is found in prelude and its"] # [doc = " name doesn't clash in current scope."] fn find_in_prelude (db : & dyn DefDatabase , local_def_map : & DefMap , item : ItemInNs , from : ModuleId ,) -> Option < Choice > { let (prelude_module , _) = local_def_map . prelude () ? ; let prelude_def_map = prelude_module . def_map (db) ; let prelude_scope = & prelude_def_map [prelude_module . local_id] . scope ; let (name , vis , _declared) = prelude_scope . name_of (item) ? ; if ! vis . is_visible_from (db , from) { return None ; } let found_and_same_def = local_def_map . with_ancestor_maps (db , from . local_id , & mut | def_map , local_id | { let per_ns = def_map [local_id] . scope . get (name) ; let same_def = match item { ItemInNs :: Types (it) => per_ns . take_types () ? == it , ItemInNs :: Values (it) => per_ns . take_values () ? == it , ItemInNs :: Macros (it) => per_ns . take_macros () ? == it , } ; Some (same_def) }) ; if found_and_same_def . unwrap_or (true) { Some (Choice :: new (false , PathKind :: Plain , name . clone () , Stable)) } else { None } }
    };
}

find_in_prelude!()