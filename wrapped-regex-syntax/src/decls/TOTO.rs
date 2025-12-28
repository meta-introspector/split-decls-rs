macro_rules! TOTO {
    () => {
        pub const TOTO : & 'static [(char , char)] = & [('ʼ' , 'ʼ') , ('𞊐' , '\u{1e2ae}')] ;
    };
}

TOTO!()