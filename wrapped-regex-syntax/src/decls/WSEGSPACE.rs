macro_rules! WSEGSPACE {
    () => {
        pub const WSEGSPACE : & 'static [(char , char)] = & [(' ' , ' ') , ('\u{1680}' , '\u{1680}') , ('\u{2000}' , '\u{2006}') , ('\u{2008}' , '\u{200a}') , ('\u{205f}' , '\u{205f}') , ('\u{3000}' , '\u{3000}') ,] ;
    };
}

WSEGSPACE!()