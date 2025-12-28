macro_rules! PARAGRAPH_SEPARATOR {
    () => {
        pub const PARAGRAPH_SEPARATOR : & 'static [(char , char)] = & [('\u{2029}' , '\u{2029}')] ;
    };
}

PARAGRAPH_SEPARATOR!()