macro_rules! deps {
    () => {
        MsvcBasicName!();
    };
}

macro_rules! impl_320 {
    () => {
        deps!();
        impl MsvcBasicName for ty :: UintTy { fn msvc_basic_name (self) -> & 'static str { match self { ty :: UintTy :: Usize => "size_t" , ty :: UintTy :: U8 => "unsigned __int8" , ty :: UintTy :: U16 => "unsigned __int16" , ty :: UintTy :: U32 => "unsigned __int32" , ty :: UintTy :: U64 => "unsigned __int64" , ty :: UintTy :: U128 => "unsigned __int128" , } } }
    };
}

impl_320!()