macro_rules! deps {
    () => {
        Blob!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl Drop for Blob < '_ > { fn drop (& mut self) { debug_assert_eq ! (self . len () , 0) ; } }
    };
}

impl_21!()