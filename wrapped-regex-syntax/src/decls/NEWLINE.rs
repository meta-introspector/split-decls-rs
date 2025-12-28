macro_rules! NEWLINE {
    () => {
        pub const NEWLINE : & 'static [(char , char)] = & [('\u{b}' , '\u{c}') , ('\u{85}' , '\u{85}') , ('\u{2028}' , '\u{2029}')] ;
    };
}

NEWLINE!()