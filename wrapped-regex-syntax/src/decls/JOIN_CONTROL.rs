macro_rules! JOIN_CONTROL {
    () => {
        pub const JOIN_CONTROL : & 'static [(char , char)] = & [('\u{200c}' , '\u{200d}')] ;
    };
}

JOIN_CONTROL!()