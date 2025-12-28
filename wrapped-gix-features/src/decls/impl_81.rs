macro_rules! deps {
    () => {
        Write!();
    };
}

macro_rules! impl_81 {
    () => {
        deps!();
        impl < T , P > io :: Seek for Write < T , P > where T : io :: Seek , { fn seek (& mut self , pos : io :: SeekFrom) -> io :: Result < u64 > { self . inner . seek (pos) } }
    };
}

impl_81!();