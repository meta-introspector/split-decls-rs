macro_rules! deps {
    () => {
        DefMap!();
        ModuleId!();
    };
}

macro_rules! is_kw_kind_relative_to_from {
    () => {
        deps!();
        fn is_kw_kind_relative_to_from (def_map : & DefMap , item : ModuleId , from : ModuleId ,) -> Option < PathKind > { if item . krate != from . krate || item . is_within_block () || from . is_within_block () { return None ; } let item = item . local_id ; let from = from . local_id ; if item == from { Some (PathKind :: SELF) } else if let Some (parent_id) = def_map [from] . parent { if item == parent_id { Some (if parent_id == DefMap :: ROOT { PathKind :: Crate } else { PathKind :: Super (1) }) } else { None } } else { None } }
    };
}

is_kw_kind_relative_to_from!();