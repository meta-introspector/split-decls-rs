macro_rules! macro_140 {
    () => {
        impl_binop_assign ! (u64x2_sse2 , AddAssign , add_assign , add) ;
    };
}

macro_140!()