macro_rules! deps {
    () => {
        CharChunks!();
    };
}

macro_rules! impl_136 {
    () => {
        deps!();
        impl < 'a > CharChunks < 'a > { fn new (s : & 'a str , n : usize) -> Self { CharChunks { s : s , n : n } } }
    };
}

impl_136!()