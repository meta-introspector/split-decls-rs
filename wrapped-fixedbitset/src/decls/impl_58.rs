macro_rules! deps {
    () => {
        Block!();
    };
}

macro_rules! impl_58 {
    () => {
        deps!();
        impl Ord for Block { # [inline] fn cmp (& self , other : & Self) -> Ordering { self . into_usize_array () . cmp (& other . into_usize_array ()) } }
    };
}

impl_58!()