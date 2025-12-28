macro_rules! deps {
    () => {
        InterruptingWriter!();
    };
}

macro_rules! impl_86 {
    () => {
        deps!();
        impl < 'a , W : Write , R : Rng > Write for InterruptingWriter < 'a , W , R > { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { if self . rng . gen_range (0.0 .. 1.0) <= self . fraction { return Err (io :: Error :: new (io :: ErrorKind :: Interrupted , "interrupted")) ; } self . w . write (buf) } fn flush (& mut self) -> io :: Result < () > { if self . rng . gen_range (0.0 .. 1.0) <= self . fraction { return Err (io :: Error :: new (io :: ErrorKind :: Interrupted , "interrupted")) ; } self . w . flush () } }
    };
}

impl_86!();