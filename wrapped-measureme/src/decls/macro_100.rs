macro_rules! macro_100 {
    () => {
        impl_serializable_string_for_fixed_size ! (10) ;
    };
}

macro_100!()