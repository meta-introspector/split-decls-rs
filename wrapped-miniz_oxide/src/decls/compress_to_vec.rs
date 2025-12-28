macro_rules! compress_to_vec {
    () => {
        # [doc = " Compress the input data to a vector, using the specified compression level (0-10)."] pub fn compress_to_vec (input : & [u8] , level : u8) -> Vec < u8 > { compress_to_vec_inner (input , level , 0 , 0) }
    };
}

compress_to_vec!();