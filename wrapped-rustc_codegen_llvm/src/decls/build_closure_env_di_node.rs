macro_rules! deps {
    () => {
        CodegenCx!();
    };
}

macro_rules! build_closure_env_di_node {
    () => {
        deps!();
        # [doc = " Builds the debuginfo node for a closure environment."] fn build_closure_env_di_node < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx > , unique_type_id : UniqueTypeId < 'tcx > ,) -> DINodeCreationResult < 'll > { let closure_env_type = unique_type_id . expect_ty () ; let & (ty :: Closure (def_id , _) | ty :: CoroutineClosure (def_id , _)) = closure_env_type . kind () else { bug ! ("build_closure_env_di_node() called with non-closure-type: {:?}" , closure_env_type) } ; let containing_scope = get_namespace_for_item (cx , def_id) ; let type_name = compute_debuginfo_type_name (cx . tcx , closure_env_type , false) ; let def_location = if cx . sess () . opts . unstable_opts . debug_info_type_line_numbers { Some (file_metadata_from_def_id (cx , Some (def_id))) } else { None } ; type_map :: build_type_with_children (cx , type_map :: stub (cx , Stub :: Struct , unique_type_id , & type_name , def_location , cx . size_and_align_of (closure_env_type) , Some (containing_scope) , DIFlags :: FlagZero ,) , | cx , owner | build_upvar_field_di_nodes (cx , closure_env_type , owner) , NO_GENERICS ,) }
    };
}

build_closure_env_di_node!();