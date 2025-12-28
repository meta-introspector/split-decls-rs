macro_rules! deps {
    () => {
        Step!();
        Bytes!();
    };
}

macro_rules! impl_97 {
    () => {
        deps!();
        impl Bytes { fn format_bytes (w : & mut dyn fmt :: Write , value : Step) -> fmt :: Result { let string = bytesize :: ByteSize (value as u64) . display () . si () . to_string () ; for token in string . split (' ') { w . write_str (token) ? ; } Ok (()) } }
    };
}

impl_97!()