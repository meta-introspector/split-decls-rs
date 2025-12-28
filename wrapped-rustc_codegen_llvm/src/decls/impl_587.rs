macro_rules! deps {
    () => {
        CodegenCx!();
    };
}

macro_rules! impl_587 {
    () => {
        deps!();
        impl < 'll , 'tcx > TypeMembershipCodegenMethods < 'tcx > for CodegenCx < 'll , 'tcx > { fn add_type_metadata (& self , function : & 'll Value , typeid : & [u8]) { let typeid_metadata = self . create_metadata (typeid) ; unsafe { let v = [llvm :: LLVMValueAsMetadata (self . const_usize (0)) , typeid_metadata] ; llvm :: LLVMRustGlobalAddMetadata (function , llvm :: MD_type as c_uint , llvm :: LLVMMDNodeInContext2 (self . llcx , v . as_ptr () , v . len ()) ,) } } fn set_type_metadata (& self , function : & 'll Value , typeid : & [u8]) { let typeid_metadata = self . create_metadata (typeid) ; unsafe { let v = [llvm :: LLVMValueAsMetadata (self . const_usize (0)) , typeid_metadata] ; llvm :: LLVMGlobalSetMetadata (function , llvm :: MD_type as c_uint , llvm :: LLVMMDNodeInContext2 (self . llcx , v . as_ptr () , v . len ()) ,) } } fn typeid_metadata (& self , typeid : & [u8]) -> Option < & 'll Metadata > { Some (self . create_metadata (typeid)) } fn add_kcfi_type_metadata (& self , function : & 'll Value , kcfi_typeid : u32) { let kcfi_type_metadata = self . const_u32 (kcfi_typeid) ; unsafe { llvm :: LLVMRustGlobalAddMetadata (function , llvm :: MD_kcfi_type as c_uint , llvm :: LLVMMDNodeInContext2 (self . llcx , & llvm :: LLVMValueAsMetadata (kcfi_type_metadata) , 1 ,) ,) } } fn set_kcfi_type_metadata (& self , function : & 'll Value , kcfi_typeid : u32) { let kcfi_type_metadata = self . const_u32 (kcfi_typeid) ; unsafe { llvm :: LLVMGlobalSetMetadata (function , llvm :: MD_kcfi_type as c_uint , llvm :: LLVMMDNodeInContext2 (self . llcx , & llvm :: LLVMValueAsMetadata (kcfi_type_metadata) , 1 ,) ,) } } }
    };
}

impl_587!()