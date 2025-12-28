macro_rules! macro_12 {
    () => {
        test ! (case : trailing_newline_chunk_boundary , text : "0123456789abcde\n" , lines : vec ! [16] , multi_byte_chars : vec ! [] ,) ;
    };
}

macro_12!();