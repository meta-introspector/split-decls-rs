macro_rules! deps {
    () => {
        CodegenCx!();
    };
}

macro_rules! is_node_local_to_unit {
    () => {
        deps!();
        pub (crate) fn is_node_local_to_unit (cx : & CodegenCx < '_ , '_ > , def_id : DefId) -> bool { ! cx . tcx . is_reachable_non_generic (def_id) }
    };
}

is_node_local_to_unit!()