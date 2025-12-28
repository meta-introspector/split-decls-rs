macro_rules! deps {
    () => {
        ConstArrayLength!();
        GenericArray!();
    };
}

macro_rules! ConstGenericArray {
    () => {
        deps!();
        # [doc = " [`GenericArray`] with a const-generic `usize` length, using the [`ConstArrayLength`] type alias for `N`."] # [doc = ""] # [doc = " To construct from a literal array, use [`from_array`](GenericArray::from_array)."] # [doc = ""] # [doc = " Note that not all `N` values are valid due to limitations inherent to `typenum` and Rust. You"] # [doc = " may need to combine [Const] with other typenum operations to get the desired length."] pub type ConstGenericArray < T , const N : usize > = GenericArray < T , ConstArrayLength < N > > ;
    };
}

ConstGenericArray!()