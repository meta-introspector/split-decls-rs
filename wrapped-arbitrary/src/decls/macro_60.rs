macro_rules! macro_60 {
    () => {
        impl_arbitrary_for_integers ! { u8 ; u16 ; u32 ; u64 ; u128 ; i8 ; i16 ; i32 ; i64 ; i128 ; }
    };
}

macro_60!();