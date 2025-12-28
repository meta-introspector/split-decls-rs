macro_rules! macro_1 {
    () => {
        test ! (case : empty_text , text : "" , lines : vec ! [] , multi_byte_chars : vec ! [] ,) ;
    };
}

macro_1!();