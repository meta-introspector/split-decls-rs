macro_rules! deps {
    () => {
        Formatter!();
    };
}

macro_rules! impl_64 {
    () => {
        deps!();
        impl Write for Formatter { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . buf . borrow_mut () . write (buf) } fn flush (& mut self) -> io :: Result < () > { self . buf . borrow_mut () . flush () } }
    };
}

impl_64!()