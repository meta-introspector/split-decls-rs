macro_rules! struct_in_context {
    () => {
        fn struct_in_context < 'll > (llcx : & 'll llvm :: Context , elts : & [& 'll Value] , packed : bool ,) -> & 'll Value { let len = c_uint :: try_from (elts . len ()) . expect ("LLVMConstStructInContext elements len overflow") ; unsafe { llvm :: LLVMConstStructInContext (llcx , elts . as_ptr () , len , packed . to_llvm_bool ()) } }
    };
}

struct_in_context!();