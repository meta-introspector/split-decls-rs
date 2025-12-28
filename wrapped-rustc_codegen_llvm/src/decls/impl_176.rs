macro_rules! deps {
    () => {
        Builder!();
    };
}

macro_rules! impl_176 {
    () => {
        deps!();
        impl < 'a , 'll , 'tcx > Builder < 'a , 'll , 'tcx > { fn align_metadata (& mut self , load : & 'll Value , align : Align) { unsafe { let md = [llvm :: LLVMValueAsMetadata (self . cx . const_u64 (align . bytes ()))] ; let md = llvm :: LLVMMDNodeInContext2 (self . cx . llcx , md . as_ptr () , md . len ()) ; self . set_metadata (load , llvm :: MD_align , md) ; } } fn noundef_metadata (& mut self , load : & 'll Value) { unsafe { let md = llvm :: LLVMMDNodeInContext2 (self . cx . llcx , ptr :: null () , 0) ; self . set_metadata (load , llvm :: MD_noundef , md) ; } } pub (crate) fn set_unpredictable (& mut self , inst : & 'll Value) { unsafe { let md = llvm :: LLVMMDNodeInContext2 (self . cx . llcx , ptr :: null () , 0) ; self . set_metadata (inst , llvm :: MD_unpredictable , md) ; } } }
    };
}

impl_176!();