macro_rules! deps {
    () => {
        CodegenCx!();
        DIB!();
    };
}

macro_rules! item_namespace {
    () => {
        deps!();
        pub (crate) fn item_namespace < 'll > (cx : & CodegenCx < 'll , '_ > , def_id : DefId) -> & 'll DIScope { if let Some (& scope) = debug_context (cx) . namespace_map . borrow () . get (& def_id) { return scope ; } let def_key = cx . tcx . def_key (def_id) ; let parent_scope = def_key . parent . map (| parent | item_namespace (cx , DefId { krate : def_id . krate , index : parent })) ; let namespace_name_string = { let mut output = String :: with_capacity (64) ; type_names :: push_item_name (cx . tcx , def_id , false , & mut output) ; output } ; let scope = unsafe { llvm :: LLVMDIBuilderCreateNameSpace (DIB (cx) , parent_scope , namespace_name_string . as_ptr () , namespace_name_string . len () , llvm :: FALSE ,) } ; debug_context (cx) . namespace_map . borrow_mut () . insert (def_id , scope) ; scope }
    };
}

item_namespace!();