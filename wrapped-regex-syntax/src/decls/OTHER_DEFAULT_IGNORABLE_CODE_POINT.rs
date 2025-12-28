macro_rules! OTHER_DEFAULT_IGNORABLE_CODE_POINT {
    () => {
        pub const OTHER_DEFAULT_IGNORABLE_CODE_POINT : & 'static [(char , char)] = & [('\u{34f}' , '\u{34f}') , ('ᅟ' , 'ᅠ') , ('\u{17b4}' , '\u{17b5}') , ('\u{2065}' , '\u{2065}') , ('ㅤ' , 'ㅤ') , ('ﾠ' , 'ﾠ') , ('\u{fff0}' , '\u{fff8}') , ('\u{e0000}' , '\u{e0000}') , ('\u{e0002}' , '\u{e001f}') , ('\u{e0080}' , '\u{e00ff}') , ('\u{e01f0}' , '\u{e0fff}') ,] ;
    };
}

OTHER_DEFAULT_IGNORABLE_CODE_POINT!()