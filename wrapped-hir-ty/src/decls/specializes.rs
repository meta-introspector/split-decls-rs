macro_rules! deps {
    () => {
        HirDatabase!();
    };
}

macro_rules! specializes {
    () => {
        deps!();
        pub (crate) fn specializes (db : & dyn HirDatabase , specializing_impl_def_id : ImplId , parent_impl_def_id : ImplId ,) -> bool { let module = specializing_impl_def_id . loc (db) . container ; let def_map = crate_def_map (db , module . krate ()) ; if ! def_map . is_unstable_feature_enabled (& sym :: specialization) && ! def_map . is_unstable_feature_enabled (& sym :: min_specialization) { return false ; } specializes_query (db , specializing_impl_def_id , parent_impl_def_id) }
    };
}

specializes!()