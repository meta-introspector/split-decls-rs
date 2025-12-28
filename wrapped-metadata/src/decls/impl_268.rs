macro_rules! deps {
    () => {
        TypeName!();
        Type!();
    };
}

macro_rules! impl_268 {
    () => {
        deps!();
        impl Type { pub fn named (namespace : & str , name : & str) -> Self { Self :: Name (TypeName :: named (namespace , name)) } pub fn code (& self) -> u8 { match self { Self :: Bool => ELEMENT_TYPE_BOOLEAN , Self :: U8 => ELEMENT_TYPE_U1 , Self :: I8 => ELEMENT_TYPE_I1 , Self :: U16 => ELEMENT_TYPE_U2 , Self :: I16 => ELEMENT_TYPE_I2 , Self :: U32 => ELEMENT_TYPE_U4 , Self :: I32 => ELEMENT_TYPE_I4 , Self :: U64 => ELEMENT_TYPE_U8 , Self :: I64 => ELEMENT_TYPE_I8 , Self :: F32 => ELEMENT_TYPE_R4 , Self :: F64 => ELEMENT_TYPE_R8 , Self :: String => ELEMENT_TYPE_STRING , Self :: AttributeEnum => 0x55 , rest => panic ! ("{rest:?}") , } } }
    };
}

impl_268!()