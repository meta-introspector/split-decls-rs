macro_rules! ZWJ {
    () => {
        pub const ZWJ : & 'static [(char , char)] = & [('\u{200d}' , '\u{200d}')] ;
    };
}

ZWJ!();