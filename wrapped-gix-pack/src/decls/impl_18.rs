macro_rules! deps {
    () => {
        LockWriter!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl io :: Seek for LockWriter { fn seek (& mut self , pos : SeekFrom) -> io :: Result < u64 > { self . writer . lock () . seek (pos) } }
    };
}

impl_18!();