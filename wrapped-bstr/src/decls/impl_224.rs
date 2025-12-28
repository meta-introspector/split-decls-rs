macro_rules! deps {
    () => {
        Chars!();
    };
}

macro_rules! impl_224 {
    () => {
        deps!();
        impl < 'a > DoubleEndedIterator for Chars < 'a > { # [inline] fn next_back (& mut self) -> Option < char > { let (ch , size) = decode_last_lossy (self . bs) ; if size == 0 { return None ; } self . bs = & self . bs [.. self . bs . len () - size] ; Some (ch) } }
    };
}

impl_224!()