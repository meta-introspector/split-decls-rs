macro_rules! SingleByteEncoder {
    () => {
        pub struct SingleByteEncoder { table : & 'static [u16 ; 128] , run_bmp_offset : usize , run_byte_offset : usize , run_length : usize , }
    };
}

SingleByteEncoder!()