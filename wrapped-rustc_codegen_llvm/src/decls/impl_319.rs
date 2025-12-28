macro_rules! deps {
    () => {
        MsvcBasicName!();
    };
}

macro_rules! impl_319 {
    () => {
        deps!();
        impl MsvcBasicName for ty :: IntTy { fn msvc_basic_name (self) -> & 'static str { match self { ty :: IntTy :: Isize => "ptrdiff_t" , ty :: IntTy :: I8 => "__int8" , ty :: IntTy :: I16 => "__int16" , ty :: IntTy :: I32 => "__int32" , ty :: IntTy :: I64 => "__int64" , ty :: IntTy :: I128 => "__int128" , } } }
    };
}

impl_319!();