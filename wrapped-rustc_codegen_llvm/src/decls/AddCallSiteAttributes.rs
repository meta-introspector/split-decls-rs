macro_rules! deps {
    () => {
        AttributePlace!();
    };
}

macro_rules! AddCallSiteAttributes {
    () => {
        deps!();
        pub (crate) fn AddCallSiteAttributes < 'll > (callsite : & 'll Value , idx : AttributePlace , attrs : & [& 'll Attribute] ,) { unsafe { LLVMRustAddCallSiteAttributes (callsite , idx . as_uint () , attrs . as_ptr () , attrs . len ()) ; } }
    };
}

AddCallSiteAttributes!()