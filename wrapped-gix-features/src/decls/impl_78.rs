macro_rules! deps {
    () => {
        Read!();
    };
}

macro_rules! impl_78 {
    () => {
        deps!();
        impl < T , P > io :: BufRead for Read < T , P > where T : io :: BufRead , P : Progress , { fn fill_buf (& mut self) -> io :: Result < & [u8] > { self . inner . fill_buf () } fn consume (& mut self , amt : usize) { self . inner . consume (amt) ; } }
    };
}

impl_78!()