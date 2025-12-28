macro_rules! deps {
    () => {
        WordsWithBreaks!();
    };
}

macro_rules! impl_209 {
    () => {
        deps!();
        impl < 'a > Iterator for WordsWithBreaks < 'a > { type Item = & 'a str ; # [inline] fn next (& mut self) -> Option < & 'a str > { let (word , size) = decode_word (self . bs) ; if size == 0 { return None ; } self . bs = & self . bs [size ..] ; Some (word) } }
    };
}

impl_209!();