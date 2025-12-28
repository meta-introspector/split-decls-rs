macro_rules! llvm_target_features {
    () => {
        fn llvm_target_features (tm : & llvm :: TargetMachine) -> Vec < (& str , & str) > { let len = unsafe { llvm :: LLVMRustGetTargetFeaturesCount (tm) } ; let mut ret = Vec :: with_capacity (len) ; for i in 0 .. len { unsafe { let mut feature = ptr :: null () ; let mut desc = ptr :: null () ; llvm :: LLVMRustGetTargetFeature (tm , i , & mut feature , & mut desc) ; if feature . is_null () || desc . is_null () { bug ! ("LLVM returned a `null` target feature string") ; } let feature = CStr :: from_ptr (feature) . to_str () . unwrap_or_else (| e | { bug ! ("LLVM returned a non-utf8 feature string: {}" , e) ; }) ; let desc = CStr :: from_ptr (desc) . to_str () . unwrap_or_else (| e | { bug ! ("LLVM returned a non-utf8 feature string: {}" , e) ; }) ; ret . push ((feature , desc)) ; } } ret }
    };
}

llvm_target_features!()