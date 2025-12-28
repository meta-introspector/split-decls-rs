macro_rules! PATTERN_WHITE_SPACE {
    () => {
        pub const PATTERN_WHITE_SPACE : & 'static [(char , char)] = & [('\t' , '\r') , (' ' , ' ') , ('\u{85}' , '\u{85}') , ('\u{200e}' , '\u{200f}') , ('\u{2028}' , '\u{2029}') ,] ;
    };
}

PATTERN_WHITE_SPACE!()