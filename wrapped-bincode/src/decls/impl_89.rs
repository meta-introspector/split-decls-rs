macro_rules! deps {
    () => {
        Reader!();
        DecodeError!();
    };
}

macro_rules! impl_89 {
    () => {
        deps!();
        impl < R > Reader for std :: io :: BufReader < R > where R : std :: io :: Read , { fn read (& mut self , bytes : & mut [u8]) -> Result < () , DecodeError > { self . read_exact (bytes) . map_err (| inner | DecodeError :: Io { inner , additional : bytes . len () , }) } # [inline] fn peek_read (& mut self , n : usize) -> Option < & [u8] > { self . buffer () . get (.. n) } # [inline] fn consume (& mut self , n : usize) { < Self as std :: io :: BufRead > :: consume (self , n) ; } }
    };
}

impl_89!();