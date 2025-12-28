macro_rules! deps {
    () => {
        PassThrough!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl < R > io :: BufRead for PassThrough < R > where R : io :: BufRead , { fn fill_buf (& mut self) -> io :: Result < & [u8] > { self . reader . fill_buf () } fn consume (& mut self , amt : usize) { self . reader . consume (amt) ; } }
    };
}

impl_14!()