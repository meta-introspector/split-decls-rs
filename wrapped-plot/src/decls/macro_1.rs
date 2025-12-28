macro_rules! macro_1 {
    () => {
        impl_data ! (f32 , f64 , i16 , i32 , i64 , i8 , isize , u16 , u32 , u64 , u8 , usize) ;
    };
}

macro_1!()