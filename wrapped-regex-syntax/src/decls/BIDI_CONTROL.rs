macro_rules! BIDI_CONTROL {
    () => {
        pub const BIDI_CONTROL : & 'static [(char , char)] = & [('\u{61c}' , '\u{61c}') , ('\u{200e}' , '\u{200f}') , ('\u{202a}' , '\u{202e}') , ('\u{2066}' , '\u{2069}') ,] ;
    };
}

BIDI_CONTROL!()