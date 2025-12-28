macro_rules! deps {
    () => {
        PotentialUtf16!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl fmt :: Debug for PotentialUtf16 { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { for c in char :: decode_utf16 (self . 0 . iter () . copied ()) { match c { Ok (c) => write ! (f , "{c}") ? , Err (e) => write ! (f , "\\0x{:x}" , e . unpaired_surrogate ()) ? , } } Ok (()) } }
    };
}

impl_37!();