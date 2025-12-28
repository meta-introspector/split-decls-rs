macro_rules! Bool {
    () => {
        # [doc = " In the LLVM-C API, boolean values are passed as `typedef int LLVMBool`,"] # [doc = " which has a different ABI from Rust or C++ `bool`."] # [doc = ""] # [doc = " This wrapper does not implement `PartialEq`."] # [doc = " To test the underlying boolean value, use [`Self::is_true`]."] # [derive (Clone , Copy)] # [repr (transparent)] pub (crate) struct Bool { value : c_int , }
    };
}

Bool!();