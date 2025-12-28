macro_rules! macro_7 {
    () => {
        test ! (case : multi_byte_char_long , text : "0123456789abcΔf012345β" , lines : vec ! [] , multi_byte_chars : vec ! [(0 , (13 , 15)) , (0 , (22 , 24))] ,) ;
    };
}

macro_7!();