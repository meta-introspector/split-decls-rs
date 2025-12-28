macro_rules! deps {
    () => {
        GenericArray!();
        ArrayLength!();
    };
}

macro_rules! impl_65 {
    () => {
        deps!();
        impl < T , N : ArrayLength > FromIterator < T > for Box < GenericArray < T , N > > { # [doc = " Create a `Box<GenericArray>` from an iterator."] # [doc = ""] # [doc = " Will panic if the number of elements is not exactly the array length."] # [doc = ""] # [doc = " See [`GenericArray::try_boxed_from_iter]` for a fallible alternative."] fn from_iter < I : IntoIterator < Item = T > > (iter : I) -> Self { match GenericArray :: try_boxed_from_iter (iter) { Ok (res) => res , Err (_) => crate :: from_iter_length_fail (N :: USIZE) , } } }
    };
}

impl_65!();