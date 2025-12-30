// Generated macro for type_di_node (function)
macro_rules! Depcrate_debuginfo_metadatatype_di_node {
() => {
// Module: crate::debuginfo::metadata
// Provides: {"type_di_node"}
// Dependencies: {}
# [doc = " Get the debuginfo node for the given type."] # [doc = ""] # [doc = " This function will look up the debuginfo node in the TypeMap. If it can't find it, it"] # [doc = " will create the node by dispatching to the corresponding `build_*_di_node()` function."] pub (crate) fn type_di_node < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx > , t : Ty < 'tcx >) -> & 'll DIType { spanned_type_di_node (cx , t , DUMMY_SP) }
};
}
