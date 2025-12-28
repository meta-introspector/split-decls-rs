macro_rules! CreateDereferenceableOrNullAttr {
    () => {
        pub (crate) fn CreateDereferenceableOrNullAttr (llcx : & Context , bytes : u64) -> & Attribute { unsafe { LLVMRustCreateDereferenceableOrNullAttr (llcx , bytes) } }
    };
}

CreateDereferenceableOrNullAttr!()