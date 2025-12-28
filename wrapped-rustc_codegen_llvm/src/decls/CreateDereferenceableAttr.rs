macro_rules! CreateDereferenceableAttr {
    () => {
        pub (crate) fn CreateDereferenceableAttr (llcx : & Context , bytes : u64) -> & Attribute { unsafe { LLVMRustCreateDereferenceableAttr (llcx , bytes) } }
    };
}

CreateDereferenceableAttr!()