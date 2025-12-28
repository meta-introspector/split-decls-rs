macro_rules! get_buf_size {
    () => {
        fn get_buf_size (dict_size : u32 , extra_size_before : u32 , extra_size_after : u32 , match_len_max : u32 ,) -> u32 { let keep_size_before = extra_size_before + dict_size ; let keep_size_after = extra_size_after + match_len_max ; let reserve_size = (dict_size / 2 + (256 << 10)) . min (512 << 20) ; keep_size_before + keep_size_after + reserve_size }
    };
}

get_buf_size!()