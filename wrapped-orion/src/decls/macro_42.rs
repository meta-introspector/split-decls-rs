macro_rules! macro_42 {
    () => {
        impl_load_into ! (u64 , u64 , from_le_bytes , load_u64_into_le) ;
    };
}

macro_42!()