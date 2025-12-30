// Generated macro for recursion_marker_type_di_node (function)
macro_rules! Depcrate_debuginfo_metadatarecursion_marker_type_di_node {
() => {
// Module: crate::debuginfo::metadata
// Provides: {"recursion_marker_type_di_node"}
// Dependencies: {}
fn recursion_marker_type_di_node < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx >) -> & 'll DIType { * debug_context (cx) . recursion_marker_type . get_or_init (move | | { create_basic_type (cx , "<recur_type>" , cx . tcx . data_layout . pointer_size () , dwarf_const :: DW_ATE_unsigned ,) }) }
};
}
