macro_rules! macro_8 {
    () => {
        test ! (case : multi_byte_char_across_chunk_boundary , text : "0123456789abcdeΔ123456789abcdef01234" , lines : vec ! [] , multi_byte_chars : vec ! [(0 , (15 , 17))] ,) ;
    };
}

macro_8!()