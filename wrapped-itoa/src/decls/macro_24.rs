macro_rules! macro_24 {
    () => {
        impl_Integer_size ! (usize as u64 # [cfg (target_pointer_width = "64")]) ;
    };
}

macro_24!()