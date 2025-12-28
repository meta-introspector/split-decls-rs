macro_rules! deps {
    () => {
        Sentences!();
    };
}

macro_rules! impl_191 {
    () => {
        deps!();
        impl < 'a > Iterator for Sentences < 'a > { type Item = & 'a str ; # [inline] fn next (& mut self) -> Option < & 'a str > { let (sentence , size) = decode_sentence (self . bs) ; if size == 0 { return None ; } self . bs = & self . bs [size ..] ; Some (sentence) } }
    };
}

impl_191!();