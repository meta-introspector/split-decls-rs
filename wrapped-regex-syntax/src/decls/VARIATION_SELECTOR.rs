macro_rules! VARIATION_SELECTOR {
    () => {
        pub const VARIATION_SELECTOR : & 'static [(char , char)] = & [('\u{180b}' , '\u{180d}') , ('\u{180f}' , '\u{180f}') , ('\u{fe00}' , '\u{fe0f}') , ('\u{e0100}' , '\u{e01ef}') ,] ;
    };
}

VARIATION_SELECTOR!();