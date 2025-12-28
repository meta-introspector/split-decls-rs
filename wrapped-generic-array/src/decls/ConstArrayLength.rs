macro_rules! deps {
    () => {
        IntoArrayLength!();
        ArrayLength!();
    };
}

macro_rules! ConstArrayLength {
    () => {
        deps!();
        # [doc = " Associated [`ArrayLength`] for one [`Const<N>`]"] # [doc = ""] # [doc = " See [`IntoArrayLength`] for more information."] # [doc = ""] # [doc = " Note that not all `N` values are valid due to limitations inherent to `typenum` and Rust. You"] # [doc = " may need to combine [Const] with other typenum operations to get the desired length."] pub type ConstArrayLength < const N : usize > = < Const < N > as IntoArrayLength > :: ArrayLength ;
    };
}

ConstArrayLength!()