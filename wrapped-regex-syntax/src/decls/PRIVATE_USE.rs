macro_rules! PRIVATE_USE {
    () => {
        pub const PRIVATE_USE : & 'static [(char , char)] = & [('\u{e000}' , '\u{f8ff}') , ('\u{f0000}' , '\u{ffffd}') , ('\u{100000}' , '\u{10fffd}') ,] ;
    };
}

PRIVATE_USE!()