macro_rules! deps {
    () => {
        Read!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl < R > io :: BufRead for Read < '_ , R > where R : io :: BufRead , { fn fill_buf (& mut self) -> io :: Result < & [u8] > { self . inner . fill_buf () } fn consume (& mut self , amt : usize) { self . inner . consume (amt) ; } }
    };
}

impl_23!();