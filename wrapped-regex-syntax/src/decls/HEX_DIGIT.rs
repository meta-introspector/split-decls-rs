macro_rules! HEX_DIGIT {
    () => {
        pub const HEX_DIGIT : & 'static [(char , char)] = & [('0' , '9') , ('A' , 'F') , ('a' , 'f') , ('０' , '９') , ('Ａ' , 'Ｆ') , ('ａ' , 'ｆ') ,] ;
    };
}

HEX_DIGIT!();