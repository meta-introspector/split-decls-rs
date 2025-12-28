macro_rules! macro_11 {
    () => {
        impl_truncate ! { u8 , u16 , u32 , u64 , u128 , usize => u8 ; u16 , u32 , u64 , u128 , usize => u16 ; u32 , u64 , u128 => u32 ; u64 , u128 => u64 ; u128 => u128 ; usize => usize ; i8 , i16 , i32 , i64 , i128 , isize => i8 ; i16 , i32 , i64 , i128 , isize => i16 ; i32 , i64 , i128 => i32 ; i64 , i128 => i64 ; i128 => i128 ; isize => isize ; }
    };
}

macro_11!()