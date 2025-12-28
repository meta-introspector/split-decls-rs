macro_rules! CreateAllocSizeAttr {
    () => {
        pub (crate) fn CreateAllocSizeAttr (llcx : & Context , size_arg : u32) -> & Attribute { unsafe { LLVMRustCreateAllocSizeAttr (llcx , size_arg) } }
    };
}

CreateAllocSizeAttr!();