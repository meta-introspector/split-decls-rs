macro_rules! deps {
    () => {
        DecompressRead!();
    };
}

macro_rules! impl_152 {
    () => {
        deps!();
        impl < R > io :: Read for DecompressRead < '_ , R > where R : io :: BufRead , { fn read (& mut self , into : & mut [u8]) -> io :: Result < usize > { gix_features :: zlib :: stream :: inflate :: read (& mut self . inner , self . decompressor , into) } }
    };
}

impl_152!();