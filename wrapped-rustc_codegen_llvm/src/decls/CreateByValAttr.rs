macro_rules! CreateByValAttr {
    () => {
        pub (crate) fn CreateByValAttr < 'll > (llcx : & 'll Context , ty : & 'll Type) -> & 'll Attribute { unsafe { LLVMRustCreateByValAttr (llcx , ty) } }
    };
}

CreateByValAttr!();