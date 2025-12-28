macro_rules! macro_25 {
    () => {
        impl_Integer_size ! (isize as i64 # [cfg (target_pointer_width = "64")]) ;
    };
}

macro_25!();