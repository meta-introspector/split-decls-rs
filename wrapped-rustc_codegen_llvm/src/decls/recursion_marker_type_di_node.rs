macro_rules! deps {
    () => {
        CodegenCx!();
    };
}

macro_rules! recursion_marker_type_di_node {
    () => {
        deps!();
        fn recursion_marker_type_di_node < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx >) -> & 'll DIType { * debug_context (cx) . recursion_marker_type . get_or_init (move | | { create_basic_type (cx , "<recur_type>" , cx . tcx . data_layout . pointer_size () , dwarf_const :: DW_ATE_unsigned ,) }) }
    };
}

recursion_marker_type_di_node!()