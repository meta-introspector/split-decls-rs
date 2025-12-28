macro_rules! deps {
    () => {
        DIB!();
        UnnamedAddr!();
        CodegenCx!();
    };
}

macro_rules! create_vtable_di_node {
    () => {
        deps!();
        # [doc = " Creates debug information for the given vtable, which is for the"] # [doc = " given type."] # [doc = ""] # [doc = " Adds the created metadata nodes directly to the crate's IR."] pub (crate) fn create_vtable_di_node < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx > , ty : Ty < 'tcx > , poly_trait_ref : Option < ty :: ExistentialTraitRef < 'tcx > > , vtable : & 'll Value ,) { if cx . dbg_cx . is_none () { return ; } if cx . sess () . opts . debuginfo != DebugInfo :: Full { return ; } let vtable = find_vtable_behind_cast (vtable) ; llvm :: set_unnamed_address (vtable , llvm :: UnnamedAddr :: No) ; let vtable_name = compute_debuginfo_vtable_name (cx . tcx , ty , poly_trait_ref , VTableNameKind :: GlobalVariable) ; let vtable_type_di_node = build_vtable_type_di_node (cx , ty , poly_trait_ref) ; let linkage_name = "" ; unsafe { llvm :: LLVMRustDIBuilderCreateStaticVariable (DIB (cx) , NO_SCOPE_METADATA , vtable_name . as_c_char_ptr () , vtable_name . len () , linkage_name . as_c_char_ptr () , linkage_name . len () , unknown_file_metadata (cx) , UNKNOWN_LINE_NUMBER , vtable_type_di_node , true , vtable , None , 0 ,) ; } }
    };
}

create_vtable_di_node!()