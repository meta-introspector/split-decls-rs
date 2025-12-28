macro_rules! deps {
    () => {
        HirDatabase!();
    };
}

macro_rules! specializes_query_cycle {
    () => {
        deps!();
        fn specializes_query_cycle (_db : & dyn HirDatabase , _salsa_id : salsa :: Id , _specializing_impl_def_id : ImplId , _parent_impl_def_id : ImplId ,) -> bool { false }
    };
}

specializes_query_cycle!()