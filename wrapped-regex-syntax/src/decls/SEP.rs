macro_rules! SEP {
    () => {
        pub const SEP : & 'static [(char , char)] = & [('\u{85}' , '\u{85}') , ('\u{2028}' , '\u{2029}')] ;
    };
}

SEP!()