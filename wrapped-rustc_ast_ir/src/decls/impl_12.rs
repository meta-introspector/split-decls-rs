macro_rules! deps {
    () => {
        UintTy!();
        IntTy!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl UintTy { pub fn name_str (& self) -> & 'static str { match * self { UintTy :: Usize => "usize" , UintTy :: U8 => "u8" , UintTy :: U16 => "u16" , UintTy :: U32 => "u32" , UintTy :: U64 => "u64" , UintTy :: U128 => "u128" , } } # [cfg (feature = "nightly")] pub fn name (self) -> Symbol { match self { UintTy :: Usize => sym :: usize , UintTy :: U8 => sym :: u8 , UintTy :: U16 => sym :: u16 , UintTy :: U32 => sym :: u32 , UintTy :: U64 => sym :: u64 , UintTy :: U128 => sym :: u128 , } } pub fn bit_width (& self) -> Option < u64 > { Some (match * self { UintTy :: Usize => return None , UintTy :: U8 => 8 , UintTy :: U16 => 16 , UintTy :: U32 => 32 , UintTy :: U64 => 64 , UintTy :: U128 => 128 , }) } pub fn normalize (& self , target_width : u16) -> Self { match self { UintTy :: Usize => match target_width { 16 => UintTy :: U16 , 32 => UintTy :: U32 , 64 => UintTy :: U64 , _ => unreachable ! () , } , _ => * self , } } pub fn to_signed (self) -> IntTy { match self { UintTy :: Usize => IntTy :: Isize , UintTy :: U8 => IntTy :: I8 , UintTy :: U16 => IntTy :: I16 , UintTy :: U32 => IntTy :: I32 , UintTy :: U64 => IntTy :: I64 , UintTy :: U128 => IntTy :: I128 , } } }
    };
}

impl_12!()