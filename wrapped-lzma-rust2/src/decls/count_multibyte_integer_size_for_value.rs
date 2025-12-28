macro_rules! count_multibyte_integer_size_for_value {
    () => {
        fn count_multibyte_integer_size_for_value (mut value : u64) -> usize { if value == 0 { return 1 ; } let mut count = 0 ; while value > 0 { count += 1 ; value >>= 7 ; } count }
    };
}

count_multibyte_integer_size_for_value!();