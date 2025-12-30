// Generated macro for build_tuple_type_di_node (function)
macro_rules! Depcrate_debuginfo_metadatabuild_tuple_type_di_node {
() => {
// Module: crate::debuginfo::metadata
// Provides: {"build_tuple_type_di_node"}
// Dependencies: {}
# [doc = " Builds the DW_TAG_structure_type debuginfo node for a Rust tuple type."] fn build_tuple_type_di_node < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx > , unique_type_id : UniqueTypeId < 'tcx > ,) -> DINodeCreationResult < 'll > { let tuple_type = unique_type_id . expect_ty () ; let & ty :: Tuple (component_types) = tuple_type . kind () else { bug ! ("build_tuple_type_di_node() called with non-tuple-type: {:?}" , tuple_type) } ; let tuple_type_and_layout = cx . layout_of (tuple_type) ; let type_name = compute_debuginfo_type_name (cx . tcx , tuple_type , false) ; type_map :: build_type_with_children (cx , type_map :: stub (cx , Stub :: Struct , unique_type_id , & type_name , None , size_and_align_of (tuple_type_and_layout) , NO_SCOPE_METADATA , DIFlags :: FlagZero ,) , | cx , tuple_di_node | { component_types . into_iter () . enumerate () . map (| (index , component_type) | { build_field_di_node (cx , tuple_di_node , & tuple_field_name (index) , cx . layout_of (component_type) , tuple_type_and_layout . fields . offset (index) , DIFlags :: FlagZero , type_di_node (cx , component_type) , None ,) }) . collect () } , NO_GENERICS ,) }
};
}
