macro_rules! deps {
    () => {
        AttributePlace!();
    };
}

macro_rules! AddFunctionAttributes {
    () => {
        deps!();
        pub (crate) fn AddFunctionAttributes < 'll > (llfn : & 'll Value , idx : AttributePlace , attrs : & [& 'll Attribute] ,) { unsafe { LLVMRustAddFunctionAttributes (llfn , idx . as_uint () , attrs . as_ptr () , attrs . len ()) ; } }
    };
}

AddFunctionAttributes!()