macro_rules! macro_21 {
    () => {
        impl_Integer_size ! (isize as i16 # [cfg (target_pointer_width = "16")]) ;
    };
}

macro_21!()