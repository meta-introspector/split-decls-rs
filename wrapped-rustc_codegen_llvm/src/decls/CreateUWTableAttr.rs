macro_rules! CreateUWTableAttr {
    () => {
        pub (crate) fn CreateUWTableAttr (llcx : & Context , async_ : bool) -> & Attribute { unsafe { LLVMRustCreateUWTableAttr (llcx , async_) } }
    };
}

CreateUWTableAttr!();