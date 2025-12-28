macro_rules! SingleByteDecoder {
    () => {
        pub struct SingleByteDecoder { table : & 'static [u16 ; 128] , }
    };
}

SingleByteDecoder!();