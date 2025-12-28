macro_rules! deps {
    () => {
        StrChunksWindows!();
        CharChunks!();
        CharWindows!();
    };
}

macro_rules! impl_134 {
    () => {
        deps!();
        impl StrChunksWindows for str { fn char_chunks (& self , n : usize) -> CharChunks { CharChunks :: new (self , n) } fn char_windows (& self , n : usize) -> CharWindows { CharWindows :: new (self , n) } }
    };
}

impl_134!()