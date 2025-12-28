macro_rules! deps {
    () => {
        LockWriter!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl io :: Read for LockWriter { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { self . writer . lock () . get_mut () . read (buf) } }
    };
}

impl_17!()