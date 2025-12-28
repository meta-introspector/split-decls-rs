macro_rules! EMOJI_COMPONENT {
    () => {
        pub const EMOJI_COMPONENT : & 'static [(char , char)] = & [('#' , '#') , ('*' , '*') , ('0' , '9') , ('\u{200d}' , '\u{200d}') , ('\u{20e3}' , '\u{20e3}') , ('\u{fe0f}' , '\u{fe0f}') , ('🇦' , '🇿') , ('🏻' , '🏿') , ('🦰' , '🦳') , ('\u{e0020}' , '\u{e007f}') ,] ;
    };
}

EMOJI_COMPONENT!();