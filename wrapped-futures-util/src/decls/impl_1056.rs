macro_rules! deps {
    () => {
        AllowStdIo!();
    };
}

macro_rules! impl_1056 {
    () => {
        deps!();
        impl < T > io :: BufRead for AllowStdIo < T > where T : io :: BufRead , { fn fill_buf (& mut self) -> io :: Result < & [u8] > { self . 0 . fill_buf () } fn consume (& mut self , amt : usize) { self . 0 . consume (amt) } }
    };
}

impl_1056!()