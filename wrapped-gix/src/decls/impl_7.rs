macro_rules! deps {
    () => {
        Read!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl < R > io :: BufRead for Read < R > where R : io :: BufRead , { fn fill_buf (& mut self) -> io :: Result < & [u8] > { self . inner . fill_buf () } fn consume (& mut self , amt : usize) { self . inner . consume (amt) ; } }
    };
}

impl_7!();