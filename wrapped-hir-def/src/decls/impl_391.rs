macro_rules! deps {
    () => {
        DefDatabase!();
        UseTree!();
        HasChildSource!();
    };
}

macro_rules! impl_391 {
    () => {
        deps!();
        impl HasChildSource < la_arena :: Idx < ast :: UseTree > > for UseId { type Value = ast :: UseTree ; fn child_source (& self , db : & dyn DefDatabase ,) -> InFile < ArenaMap < la_arena :: Idx < ast :: UseTree > , Self :: Value > > { let loc = self . lookup (db) ; InFile :: new (loc . id . file_id , use_tree_source_map (db , loc . id) . into_iter () . collect ()) } }
    };
}

impl_391!()