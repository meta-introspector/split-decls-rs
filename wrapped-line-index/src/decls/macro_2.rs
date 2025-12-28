macro_rules! macro_2 {
    () => {
        test ! (case : newlines_short , text : "a\nc" , lines : vec ! [2] , multi_byte_chars : vec ! [] ,) ;
    };
}

macro_2!();