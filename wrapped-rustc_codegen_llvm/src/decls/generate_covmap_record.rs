macro_rules! deps {
    () => {
        Linkage!();
        CodegenCx!();
        CovmapVersion!();
    };
}

macro_rules! generate_covmap_record {
    () => {
        deps!();
        # [doc = " Generates the contents of the covmap record for this CGU, which mostly"] # [doc = " consists of a header and a list of filenames. The record is then stored"] # [doc = " as a global variable in the `__llvm_covmap` section."] fn generate_covmap_record < 'll > (cx : & mut CodegenCx < 'll , '_ > , version : CovmapVersion , filenames_buffer : & [u8] ,) { let covmap_header = cx . const_struct (& [cx . const_u32 (0) , cx . const_u32 (filenames_buffer . len () as u32) , cx . const_u32 (0) , cx . const_u32 (version . to_u32 ()) ,] , false ,) ; let covmap_record = cx . const_struct (& [covmap_header , cx . const_bytes (filenames_buffer)] , false) ; let covmap_global = llvm :: add_global (cx . llmod , cx . val_ty (covmap_record) , & llvm_cov :: covmap_var_name ()) ; llvm :: set_initializer (covmap_global , covmap_record) ; llvm :: set_global_constant (covmap_global , true) ; llvm :: set_linkage (covmap_global , llvm :: Linkage :: PrivateLinkage) ; llvm :: set_section (covmap_global , & llvm_cov :: covmap_section_name (cx . llmod)) ; llvm :: set_alignment (covmap_global , Align :: EIGHT) ; cx . add_used_global (covmap_global) ; }
    };
}

generate_covmap_record!()