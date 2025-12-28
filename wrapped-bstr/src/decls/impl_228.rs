macro_rules! deps {
    () => {
        CharIndices!();
    };
}

macro_rules! impl_228 {
    () => {
        deps!();
        impl < 'a > DoubleEndedIterator for CharIndices < 'a > { # [inline] fn next_back (& mut self) -> Option < (usize , usize , char) > { let (ch , size) = decode_last_lossy (self . bs) ; if size == 0 { return None ; } self . bs = & self . bs [.. self . bs . len () - size] ; self . reverse_index -= size ; Some ((self . reverse_index , self . reverse_index + size , ch)) } }
    };
}

impl_228!()