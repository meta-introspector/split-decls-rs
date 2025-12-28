macro_rules! deps {
    () => {
        Write!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl < W > io :: Seek for Write < '_ , W > where W : std :: io :: Seek , { fn seek (& mut self , pos : io :: SeekFrom) -> io :: Result < u64 > { self . inner . seek (pos) } }
    };
}

impl_26!()