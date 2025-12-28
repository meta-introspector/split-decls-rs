macro_rules! deps {
    () => {
        DIB!();
        CodegenCx!();
    };
}

macro_rules! build_global_var_di_node {
    () => {
        deps!();
        # [doc = " Creates debug information for the given global variable."] # [doc = ""] # [doc = " Adds the created debuginfo nodes directly to the crate's IR."] pub (crate) fn build_global_var_di_node < 'll > (cx : & CodegenCx < 'll , '_ > , def_id : DefId , global : & 'll Value ,) { if cx . dbg_cx . is_none () { return ; } if cx . sess () . opts . debuginfo != DebugInfo :: Full { return ; } let tcx = cx . tcx ; let var_scope = get_namespace_for_item (cx , def_id) ; let (file_metadata , line_number) = file_metadata_from_def_id (cx , Some (def_id)) ; let is_local_to_unit = is_node_local_to_unit (cx , def_id) ; let DefKind :: Static { nested , .. } = cx . tcx . def_kind (def_id) else { bug ! () } ; if nested { return ; } let variable_type = Instance :: mono (cx . tcx , def_id) . ty (cx . tcx , cx . typing_env ()) ; let type_di_node = type_di_node (cx , variable_type) ; let var_name = tcx . item_name (def_id) ; let var_name = var_name . as_str () ; let linkage_name = mangled_name_of_instance (cx , Instance :: mono (tcx , def_id)) . name ; let linkage_name = if var_name == linkage_name { "" } else { linkage_name } ; let global_align = cx . align_of (variable_type) ; unsafe { llvm :: LLVMRustDIBuilderCreateStaticVariable (DIB (cx) , Some (var_scope) , var_name . as_c_char_ptr () , var_name . len () , linkage_name . as_c_char_ptr () , linkage_name . len () , file_metadata , line_number , type_di_node , is_local_to_unit , global , None , global_align . bits () as u32 ,) ; } }
    };
}

build_global_var_di_node!();