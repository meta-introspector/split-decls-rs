macro_rules! deps {
    () => {
        BuiltinInt!();
        BuiltinType!();
        BuiltinFloat!();
        BuiltinUint!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl BuiltinType { # [rustfmt :: skip] pub fn all_builtin_types () -> [(Name , BuiltinType) ; 19] { [(Name :: new_symbol_root (sym :: char) , BuiltinType :: Char) , (Name :: new_symbol_root (sym :: bool) , BuiltinType :: Bool) , (Name :: new_symbol_root (sym :: str) , BuiltinType :: Str) , (Name :: new_symbol_root (sym :: isize) , BuiltinType :: Int (BuiltinInt :: Isize)) , (Name :: new_symbol_root (sym :: i8) , BuiltinType :: Int (BuiltinInt :: I8)) , (Name :: new_symbol_root (sym :: i16) , BuiltinType :: Int (BuiltinInt :: I16)) , (Name :: new_symbol_root (sym :: i32) , BuiltinType :: Int (BuiltinInt :: I32)) , (Name :: new_symbol_root (sym :: i64) , BuiltinType :: Int (BuiltinInt :: I64)) , (Name :: new_symbol_root (sym :: i128) , BuiltinType :: Int (BuiltinInt :: I128)) , (Name :: new_symbol_root (sym :: usize) , BuiltinType :: Uint (BuiltinUint :: Usize)) , (Name :: new_symbol_root (sym :: u8) , BuiltinType :: Uint (BuiltinUint :: U8)) , (Name :: new_symbol_root (sym :: u16) , BuiltinType :: Uint (BuiltinUint :: U16)) , (Name :: new_symbol_root (sym :: u32) , BuiltinType :: Uint (BuiltinUint :: U32)) , (Name :: new_symbol_root (sym :: u64) , BuiltinType :: Uint (BuiltinUint :: U64)) , (Name :: new_symbol_root (sym :: u128) , BuiltinType :: Uint (BuiltinUint :: U128)) , (Name :: new_symbol_root (sym :: f16) , BuiltinType :: Float (BuiltinFloat :: F16)) , (Name :: new_symbol_root (sym :: f32) , BuiltinType :: Float (BuiltinFloat :: F32)) , (Name :: new_symbol_root (sym :: f64) , BuiltinType :: Float (BuiltinFloat :: F64)) , (Name :: new_symbol_root (sym :: f128) , BuiltinType :: Float (BuiltinFloat :: F128)) ,] } pub fn by_name (name : & Name) -> Option < Self > { Self :: all_builtin_types () . iter () . find_map (| (n , ty) | if n == name { Some (* ty) } else { None }) } }
    };
}

impl_39!()