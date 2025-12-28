macro_rules! ptr_size_bits {
    () => {
        pub const fn ptr_size_bits () -> usize { mem :: size_of :: < usize > () * 8 }
    };
}

ptr_size_bits!()