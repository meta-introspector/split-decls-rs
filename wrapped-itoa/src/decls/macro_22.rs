macro_rules! macro_22 {
    () => {
        impl_Integer_size ! (usize as u16 # [cfg (target_pointer_width = "16")]) ;
    };
}

macro_22!()