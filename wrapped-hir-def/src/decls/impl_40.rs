macro_rules! deps {
    () => {
        BuiltinInt!();
        BuiltinFloat!();
        BuiltinType!();
        BuiltinUint!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl AsName for BuiltinType { fn as_name (& self) -> Name { match self { BuiltinType :: Char => Name :: new_symbol_root (sym :: char) , BuiltinType :: Bool => Name :: new_symbol_root (sym :: bool) , BuiltinType :: Str => Name :: new_symbol_root (sym :: str) , BuiltinType :: Int (it) => match it { BuiltinInt :: Isize => Name :: new_symbol_root (sym :: isize) , BuiltinInt :: I8 => Name :: new_symbol_root (sym :: i8) , BuiltinInt :: I16 => Name :: new_symbol_root (sym :: i16) , BuiltinInt :: I32 => Name :: new_symbol_root (sym :: i32) , BuiltinInt :: I64 => Name :: new_symbol_root (sym :: i64) , BuiltinInt :: I128 => Name :: new_symbol_root (sym :: i128) , } , BuiltinType :: Uint (it) => match it { BuiltinUint :: Usize => Name :: new_symbol_root (sym :: usize) , BuiltinUint :: U8 => Name :: new_symbol_root (sym :: u8) , BuiltinUint :: U16 => Name :: new_symbol_root (sym :: u16) , BuiltinUint :: U32 => Name :: new_symbol_root (sym :: u32) , BuiltinUint :: U64 => Name :: new_symbol_root (sym :: u64) , BuiltinUint :: U128 => Name :: new_symbol_root (sym :: u128) , } , BuiltinType :: Float (it) => match it { BuiltinFloat :: F16 => Name :: new_symbol_root (sym :: f16) , BuiltinFloat :: F32 => Name :: new_symbol_root (sym :: f32) , BuiltinFloat :: F64 => Name :: new_symbol_root (sym :: f64) , BuiltinFloat :: F128 => Name :: new_symbol_root (sym :: f128) , } , } } }
    };
}

impl_40!();