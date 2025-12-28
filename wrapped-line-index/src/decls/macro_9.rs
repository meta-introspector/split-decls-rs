macro_rules! macro_9 {
    () => {
        test ! (case : multi_byte_char_across_chunk_boundary_tail , text : "0123456789abcdeΔ...." , lines : vec ! [] , multi_byte_chars : vec ! [(0 , (15 , 17))] ,) ;
    };
}

macro_9!();