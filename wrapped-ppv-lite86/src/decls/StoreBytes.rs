macro_rules! StoreBytes {
    () => {
        pub trait StoreBytes { # [doc = " # Safety"] # [doc = " Caller must ensure the type of Self is appropriate for the hardware of the execution"] # [doc = " environment."] unsafe fn unsafe_read_le (input : & [u8]) -> Self ; # [doc = " # Safety"] # [doc = " Caller must ensure the type of Self is appropriate for the hardware of the execution"] # [doc = " environment."] unsafe fn unsafe_read_be (input : & [u8]) -> Self ; fn write_le (self , out : & mut [u8]) ; fn write_be (self , out : & mut [u8]) ; }
    };
}

StoreBytes!();