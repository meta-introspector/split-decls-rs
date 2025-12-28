macro_rules! deps {
    () => {
        AttributeKind!();
    };
}

macro_rules! impl_515 {
    () => {
        deps!();
        impl AttributeKind { # [doc = " Create an LLVM Attribute with no associated value."] pub (crate) fn create_attr (self , llcx : & Context) -> & Attribute { unsafe { LLVMRustCreateAttrNoValue (llcx , self) } } }
    };
}

impl_515!()