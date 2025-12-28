macro_rules! macro_22 {
    () => {
        impl_Integer_size ! (usize as u32 # [cfg (target_pointer_width = "32")]) ;
    };
}

macro_22!()