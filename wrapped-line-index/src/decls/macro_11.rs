macro_rules! macro_11 {
    () => {
        test ! (case : trailing_newline , text : "0123456789\n" , lines : vec ! [11] , multi_byte_chars : vec ! [] ,) ;
    };
}

macro_11!()