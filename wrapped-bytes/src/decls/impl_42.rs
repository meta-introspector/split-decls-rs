macro_rules! deps {
    () => {
        Buf!();
        Reader!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl < B : Buf + Sized > io :: BufRead for Reader < B > { fn fill_buf (& mut self) -> io :: Result < & [u8] > { Ok (self . buf . chunk ()) } fn consume (& mut self , amt : usize) { self . buf . advance (amt) } }
    };
}

impl_42!();