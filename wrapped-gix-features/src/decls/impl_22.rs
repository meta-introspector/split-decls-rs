macro_rules! deps {
    () => {
        Read!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl < R > io :: Read for Read < '_ , R > where R : io :: Read , { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { if self . should_interrupt . load (Ordering :: Relaxed) { return Err (std :: io :: Error :: other ("Interrupted")) ; } self . inner . read (buf) } }
    };
}

impl_22!();