macro_rules! deps {
    () => {
        IntTy!();
        UintTy!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl IntTy { pub fn name_str (& self) -> & 'static str { match * self { IntTy :: Isize => "isize" , IntTy :: I8 => "i8" , IntTy :: I16 => "i16" , IntTy :: I32 => "i32" , IntTy :: I64 => "i64" , IntTy :: I128 => "i128" , } } # [cfg (feature = "nightly")] pub fn name (self) -> Symbol { match self { IntTy :: Isize => sym :: isize , IntTy :: I8 => sym :: i8 , IntTy :: I16 => sym :: i16 , IntTy :: I32 => sym :: i32 , IntTy :: I64 => sym :: i64 , IntTy :: I128 => sym :: i128 , } } pub fn bit_width (& self) -> Option < u64 > { Some (match * self { IntTy :: Isize => return None , IntTy :: I8 => 8 , IntTy :: I16 => 16 , IntTy :: I32 => 32 , IntTy :: I64 => 64 , IntTy :: I128 => 128 , }) } pub fn normalize (& self , target_width : u16) -> Self { match self { IntTy :: Isize => match target_width { 16 => IntTy :: I16 , 32 => IntTy :: I32 , 64 => IntTy :: I64 , _ => unreachable ! () , } , _ => * self , } } pub fn to_unsigned (self) -> UintTy { match self { IntTy :: Isize => UintTy :: Usize , IntTy :: I8 => UintTy :: U8 , IntTy :: I16 => UintTy :: U16 , IntTy :: I32 => UintTy :: U32 , IntTy :: I64 => UintTy :: U64 , IntTy :: I128 => UintTy :: U128 , } } }
    };
}

impl_2!()