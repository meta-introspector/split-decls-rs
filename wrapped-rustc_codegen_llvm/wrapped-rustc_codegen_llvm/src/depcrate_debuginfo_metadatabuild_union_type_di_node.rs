// Generated macro for build_union_type_di_node (function)
macro_rules! Depcrate_debuginfo_metadatabuild_union_type_di_node {
() => {
// Module: crate::debuginfo::metadata
// Provides: {"build_union_type_di_node"}
// Dependencies: {}
# [doc = " Build the debuginfo node for a Rust `union` type."] fn build_union_type_di_node < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx > , unique_type_id : UniqueTypeId < 'tcx > ,) -> DINodeCreationResult < 'll > { let union_type = unique_type_id . expect_ty () ; let (union_def_id , variant_def) = match union_type . kind () { ty :: Adt (def , _) => (def . did () , def . non_enum_variant ()) , _ => bug ! ("build_union_type_di_node on a non-ADT") , } ; let containing_scope = get_namespace_for_item (cx , union_def_id) ; let union_ty_and_layout = cx . layout_of (union_type) ; let type_name = compute_debuginfo_type_name (cx . tcx , union_type , false) ; let def_location = if cx . sess () . opts . unstable_opts . debug_info_type_line_numbers { Some (file_metadata_from_def_id (cx , Some (union_def_id))) } else { None } ; type_map :: build_type_with_children (cx , type_map :: stub (cx , Stub :: Union , unique_type_id , & type_name , def_location , size_and_align_of (union_ty_and_layout) , Some (containing_scope) , DIFlags :: FlagZero ,) , | cx , owner | { variant_def . fields . iter () . enumerate () . map (| (i , f) | { let field_layout = union_ty_and_layout . field (cx , i) ; let def_id = if cx . sess () . opts . unstable_opts . debug_info_type_line_numbers { Some (f . did) } else { None } ; build_field_di_node (cx , owner , f . name . as_str () , field_layout , Size :: ZERO , DIFlags :: FlagZero , type_di_node (cx , field_layout . ty) , def_id ,) }) . collect () } , | cx | build_generic_type_param_di_nodes (cx , union_type) ,) }
};
}
