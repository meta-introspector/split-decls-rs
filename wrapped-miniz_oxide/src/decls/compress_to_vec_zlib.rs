macro_rules! compress_to_vec_zlib {
    () => {
        # [doc = " Compress the input data to a vector, using the specified compression level (0-10), and with a"] # [doc = " zlib wrapper."] pub fn compress_to_vec_zlib (input : & [u8] , level : u8) -> Vec < u8 > { compress_to_vec_inner (input , level , 1 , 0) }
    };
}

compress_to_vec_zlib!()