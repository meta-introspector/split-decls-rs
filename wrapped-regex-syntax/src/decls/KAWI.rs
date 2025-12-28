macro_rules! KAWI {
    () => {
        pub const KAWI : & 'static [(char , char)] = & [('\u{11f00}' , '𑼐') , ('𑼒' , '\u{11f3a}') , ('𑼾' , '\u{11f5a}')] ;
    };
}

KAWI!()