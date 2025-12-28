macro_rules! FILE_MAGIC_TOP_LEVEL {
    () => {
        pub const FILE_MAGIC_TOP_LEVEL : & [u8 ; 4] = b"MMPD" ;
    };
}

FILE_MAGIC_TOP_LEVEL!();