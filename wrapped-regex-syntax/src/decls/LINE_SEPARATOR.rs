macro_rules! LINE_SEPARATOR {
    () => {
        pub const LINE_SEPARATOR : & 'static [(char , char)] = & [('\u{2028}' , '\u{2028}')] ;
    };
}

LINE_SEPARATOR!();