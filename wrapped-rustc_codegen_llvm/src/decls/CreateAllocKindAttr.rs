macro_rules! CreateAllocKindAttr {
    () => {
        pub (crate) fn CreateAllocKindAttr (llcx : & Context , kind_arg : AllocKindFlags) -> & Attribute { unsafe { LLVMRustCreateAllocKindAttr (llcx , kind_arg . bits ()) } }
    };
}

CreateAllocKindAttr!();