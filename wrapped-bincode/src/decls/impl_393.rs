macro_rules! deps {
    () => {
        DecodeError!();
        Reader!();
    };
}

macro_rules! impl_393 {
    () => {
        deps!();
        impl < T > Reader for & mut T where T : Reader , { # [inline] fn read (& mut self , bytes : & mut [u8]) -> Result < () , DecodeError > { (* * self) . read (bytes) } # [inline] fn peek_read (& mut self , n : usize) -> Option < & [u8] > { (* * self) . peek_read (n) } # [inline] fn consume (& mut self , n : usize) { (* self) . consume (n) } }
    };
}

impl_393!()