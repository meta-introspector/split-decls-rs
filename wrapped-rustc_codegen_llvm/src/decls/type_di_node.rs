macro_rules! deps {
    () => {
        CodegenCx!();
    };
}

macro_rules! type_di_node {
    () => {
        deps!();
        # [doc = " Get the debuginfo node for the given type."] # [doc = ""] # [doc = " This function will look up the debuginfo node in the TypeMap. If it can't find it, it"] # [doc = " will create the node by dispatching to the corresponding `build_*_di_node()` function."] pub (crate) fn type_di_node < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx > , t : Ty < 'tcx >) -> & 'll DIType { spanned_type_di_node (cx , t , DUMMY_SP) }
    };
}

type_di_node!()