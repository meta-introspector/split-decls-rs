macro_rules! deps {
    () => {
        Chars!();
    };
}

macro_rules! impl_223 {
    () => {
        deps!();
        impl < 'a > Iterator for Chars < 'a > { type Item = char ; # [inline] fn next (& mut self) -> Option < char > { let (ch , size) = decode_lossy (self . bs) ; if size == 0 { return None ; } self . bs = & self . bs [size ..] ; Some (ch) } }
    };
}

impl_223!();