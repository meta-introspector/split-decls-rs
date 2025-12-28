macro_rules! get_param {
    () => {
        # [doc = " Safe wrapper around `LLVMGetParam`, because segfaults are no fun."] pub (crate) fn get_param (llfn : & Value , index : c_uint) -> & Value { unsafe { assert ! (index < LLVMCountParams (llfn) , "out of bounds argument access: {} out of {} arguments" , index , LLVMCountParams (llfn)) ; LLVMGetParam (llfn , index) } }
    };
}

get_param!()