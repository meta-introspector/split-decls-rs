macro_rules! deps {
    () => {
        LockWriter!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl io :: Write for LockWriter { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . writer . lock () . write (buf) } fn flush (& mut self) -> io :: Result < () > { self . writer . lock () . flush () } }
    };
}

impl_16!()