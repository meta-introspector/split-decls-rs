macro_rules! deps {
    () => {
        Visibility!();
        MetadataType!();
        CodegenCx!();
    };
}

macro_rules! apply_vcall_visibility_metadata {
    () => {
        deps!();
        pub (crate) fn apply_vcall_visibility_metadata < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx > , ty : Ty < 'tcx > , trait_ref : Option < ExistentialTraitRef < 'tcx > > , vtable : & 'll Value ,) { if ! cx . sess () . opts . unstable_opts . virtual_function_elimination || cx . sess () . lto () != Lto :: Fat { return ; } enum VCallVisibility { Public = 0 , LinkageUnit = 1 , TranslationUnit = 2 , } let Some (trait_ref) = trait_ref else { return } ; let vtable = find_vtable_behind_cast (vtable) ; let trait_ref_self = trait_ref . with_self_ty (cx . tcx , ty) ; let trait_ref_self = cx . tcx . erase_and_anonymize_regions (trait_ref_self) ; let trait_def_id = trait_ref_self . def_id ; let trait_vis = cx . tcx . visibility (trait_def_id) ; let cgus = cx . sess () . codegen_units () . as_usize () ; let single_cgu = cgus == 1 ; let lto = cx . sess () . lto () ; let vcall_visibility = match (lto , trait_vis , single_cgu) { (Lto :: No | Lto :: ThinLocal , Visibility :: Public , _) | (Lto :: No , Visibility :: Restricted (_) , false) => VCallVisibility :: Public , (Lto :: Fat | Lto :: Thin , Visibility :: Public , _) | (Lto :: ThinLocal | Lto :: Thin | Lto :: Fat , Visibility :: Restricted (_) , false) => { VCallVisibility :: LinkageUnit } (_ , Visibility :: Restricted (_) , true) => VCallVisibility :: TranslationUnit , } ; let trait_ref_typeid = typeid_for_trait_ref (cx . tcx , trait_ref) ; let typeid = cx . create_metadata (trait_ref_typeid . as_bytes ()) ; unsafe { let v = [llvm :: LLVMValueAsMetadata (cx . const_usize (0)) , typeid] ; llvm :: LLVMRustGlobalAddMetadata (vtable , llvm :: MD_type as c_uint , llvm :: LLVMMDNodeInContext2 (cx . llcx , v . as_ptr () , v . len ()) ,) ; let vcall_visibility = llvm :: LLVMValueAsMetadata (cx . const_u64 (vcall_visibility as u64)) ; let vcall_visibility_metadata = llvm :: LLVMMDNodeInContext2 (cx . llcx , & vcall_visibility , 1) ; llvm :: LLVMGlobalSetMetadata (vtable , llvm :: MetadataType :: MD_vcall_visibility as c_uint , vcall_visibility_metadata ,) ; } }
    };
}

apply_vcall_visibility_metadata!()