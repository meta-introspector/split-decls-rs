macro_rules! deps {
    () => {
        CharIndices!();
    };
}

macro_rules! impl_227 {
    () => {
        deps!();
        impl < 'a > Iterator for CharIndices < 'a > { type Item = (usize , usize , char) ; # [inline] fn next (& mut self) -> Option < (usize , usize , char) > { let index = self . forward_index ; let (ch , size) = decode_lossy (self . bs) ; if size == 0 { return None ; } self . bs = & self . bs [size ..] ; self . forward_index += size ; Some ((index , index + size , ch)) } }
    };
}

impl_227!()