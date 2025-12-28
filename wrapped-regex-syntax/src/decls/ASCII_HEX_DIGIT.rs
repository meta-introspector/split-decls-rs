macro_rules! ASCII_HEX_DIGIT {
    () => {
        pub const ASCII_HEX_DIGIT : & 'static [(char , char)] = & [('0' , '9') , ('A' , 'F') , ('a' , 'f')] ;
    };
}

ASCII_HEX_DIGIT!()