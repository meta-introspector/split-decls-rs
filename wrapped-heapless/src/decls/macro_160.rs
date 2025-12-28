macro_rules! macro_160 {
    () => {
        impl_lentype ! (u8 , u16 , # [cfg (any (target_pointer_width = "32" , target_pointer_width = "64"))] u32 , usize) ;
    };
}

macro_160!()