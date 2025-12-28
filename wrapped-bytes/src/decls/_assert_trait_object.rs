macro_rules! deps {
    () => {
        BufMut!();
    };
}

macro_rules! _assert_trait_object {
    () => {
        deps!();
        fn _assert_trait_object (_b : & dyn BufMut) { }
    };
}

_assert_trait_object!()