macro_rules! deps {
    () => {
        CharStr!();
    };
}

macro_rules! impl_143 {
    () => {
        deps!();
        impl CharStr { # [doc = " Create a new string from `c`."] pub fn new (c : char) -> CharStr { let mut self_ = CharStr { buf : [0 ; 4] , len : c . len_utf8 () as u32 , } ; let _ = crate :: char :: encode_utf8 (c , & mut self_ . buf) ; self_ } }
    };
}

impl_143!()