macro_rules! CreateStructRetAttr {
    () => {
        pub (crate) fn CreateStructRetAttr < 'll > (llcx : & 'll Context , ty : & 'll Type) -> & 'll Attribute { unsafe { LLVMRustCreateStructRetAttr (llcx , ty) } }
    };
}

CreateStructRetAttr!();