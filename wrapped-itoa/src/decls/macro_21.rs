macro_rules! macro_21 {
    () => {
        impl_Integer_size ! (isize as i32 # [cfg (target_pointer_width = "32")]) ;
    };
}

macro_21!()