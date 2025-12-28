macro_rules! macro_6 {
    () => {
        test ! (case : multi_byte_char_short , text : "aβc" , lines : vec ! [] , multi_byte_chars : vec ! [(0 , (1 , 3))] ,) ;
    };
}

macro_6!()