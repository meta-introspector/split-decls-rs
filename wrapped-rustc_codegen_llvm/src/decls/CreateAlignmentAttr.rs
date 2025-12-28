macro_rules! CreateAlignmentAttr {
    () => {
        pub (crate) fn CreateAlignmentAttr (llcx : & Context , bytes : u64) -> & Attribute { unsafe { LLVMRustCreateAlignmentAttr (llcx , bytes) } }
    };
}

CreateAlignmentAttr!()