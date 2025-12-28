macro_rules! macro_10 {
    () => {
        impl_extend ! { u8 => u8 , u16 , u32 , u64 , u128 , usize ; u16 => u16 , u32 , u64 , u128 , usize ; u32 => u32 , u64 , u128 ; u64 => u64 , u128 ; u128 => u128 ; usize => usize ; i8 => i8 , i16 , i32 , i64 , i128 , isize ; i16 => i16 , i32 , i64 , i128 , isize ; i32 => i32 , i64 , i128 ; i64 => i64 , i128 ; i128 => i128 ; isize => isize ; }
    };
}

macro_10!()