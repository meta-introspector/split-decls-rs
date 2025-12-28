macro_rules! deps {
    () => {
        DecodeError!();
        Reader!();
        IoReader!();
    };
}

macro_rules! impl_88 {
    () => {
        deps!();
        impl < R > Reader for IoReader < R > where R : std :: io :: Read , { # [inline (always)] fn read (& mut self , bytes : & mut [u8]) -> Result < () , DecodeError > { self . reader . read_exact (bytes) . map_err (| inner | DecodeError :: Io { inner , additional : bytes . len () , }) } }
    };
}

impl_88!();