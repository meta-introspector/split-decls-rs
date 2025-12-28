macro_rules! macro_64 {
    () => {
        impl_arbitrary_for_floats ! { f32 : u32 ; f64 : u64 ; }
    };
}

macro_64!()