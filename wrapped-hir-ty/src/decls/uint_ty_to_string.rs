macro_rules! uint_ty_to_string {
    () => {
        pub fn uint_ty_to_string (ty : UintTy) -> & 'static str { match ty { UintTy :: Usize => "usize" , UintTy :: U8 => "u8" , UintTy :: U16 => "u16" , UintTy :: U32 => "u32" , UintTy :: U64 => "u64" , UintTy :: U128 => "u128" , } }
    };
}

uint_ty_to_string!()