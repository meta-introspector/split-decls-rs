macro_rules! deps {
    () => {
        BuiltinUint!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl fmt :: Display for BuiltinUint { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (match self { BuiltinUint :: Usize => "usize" , BuiltinUint :: U8 => "u8" , BuiltinUint :: U16 => "u16" , BuiltinUint :: U32 => "u32" , BuiltinUint :: U64 => "u64" , BuiltinUint :: U128 => "u128" , }) } }
    };
}

impl_46!()