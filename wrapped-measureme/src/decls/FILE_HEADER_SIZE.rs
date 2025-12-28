macro_rules! FILE_HEADER_SIZE {
    () => {
        # [doc = " The size of the file header in bytes. Note that functions in this module"] # [doc = " rely on this size to be `8`."] pub const FILE_HEADER_SIZE : usize = 8 ;
    };
}

FILE_HEADER_SIZE!()