macro_rules! macro_19 {
    () => {
        impl_Integer_size ! (isize as i16 # [cfg (target_pointer_width = "16")]) ;
    };
}

macro_19!()