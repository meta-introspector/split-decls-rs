macro_rules! macro_42 {
    () => {
        impl_const_get_with_index_for_integer ! (i128) ;
    };
}

macro_42!()