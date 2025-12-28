macro_rules! named_struct {
    () => {
        pub (crate) fn named_struct < 'll > (ty : & 'll Type , elts : & [& 'll Value]) -> & 'll Value { let len = c_uint :: try_from (elts . len ()) . expect ("LLVMConstStructInContext elements len overflow") ; unsafe { llvm :: LLVMConstNamedStruct (ty , elts . as_ptr () , len) } }
    };
}

named_struct!();