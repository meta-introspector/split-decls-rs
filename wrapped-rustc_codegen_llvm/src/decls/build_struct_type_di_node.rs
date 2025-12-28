macro_rules! deps {
    () => {
        CodegenCx!();
    };
}

macro_rules! build_struct_type_di_node {
    () => {
        deps!();
        # [doc = " Creates the debuginfo node for a Rust struct type. Maybe be a regular struct or a tuple-struct."] fn build_struct_type_di_node < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx > , unique_type_id : UniqueTypeId < 'tcx > ,) -> DINodeCreationResult < 'll > { let struct_type = unique_type_id . expect_ty () ; let ty :: Adt (adt_def , _) = struct_type . kind () else { bug ! ("build_struct_type_di_node() called with non-struct-type: {:?}" , struct_type) ; } ; assert ! (adt_def . is_struct ()) ; let containing_scope = get_namespace_for_item (cx , adt_def . did ()) ; let struct_type_and_layout = cx . layout_of (struct_type) ; let variant_def = adt_def . non_enum_variant () ; let def_location = if cx . sess () . opts . unstable_opts . debug_info_type_line_numbers { Some (file_metadata_from_def_id (cx , Some (adt_def . did ()))) } else { None } ; type_map :: build_type_with_children (cx , type_map :: stub (cx , Stub :: Struct , unique_type_id , & compute_debuginfo_type_name (cx . tcx , struct_type , false) , def_location , size_and_align_of (struct_type_and_layout) , Some (containing_scope) , visibility_di_flags (cx , adt_def . did () , adt_def . did ()) ,) , | cx , owner | { variant_def . fields . iter () . enumerate () . map (| (i , f) | { let field_name = if variant_def . ctor_kind () == Some (CtorKind :: Fn) { tuple_field_name (i) } else { Cow :: Borrowed (f . name . as_str ()) } ; let field_layout = struct_type_and_layout . field (cx , i) ; let def_id = if cx . sess () . opts . unstable_opts . debug_info_type_line_numbers { Some (f . did) } else { None } ; build_field_di_node (cx , owner , & field_name [..] , field_layout , struct_type_and_layout . fields . offset (i) , visibility_di_flags (cx , f . did , adt_def . did ()) , type_di_node (cx , field_layout . ty) , def_id ,) }) . collect () } , | cx | build_generic_type_param_di_nodes (cx , struct_type) ,) }
    };
}

build_struct_type_di_node!()