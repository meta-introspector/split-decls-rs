macro_rules! deps {
    () => {
        CodegenCx!();
    };
}

macro_rules! get_namespace_for_item {
    () => {
        deps!();
        pub (crate) fn get_namespace_for_item < 'll > (cx : & CodegenCx < 'll , '_ > , def_id : DefId) -> & 'll DIScope { item_namespace (cx , cx . tcx . parent (def_id)) }
    };
}

get_namespace_for_item!();