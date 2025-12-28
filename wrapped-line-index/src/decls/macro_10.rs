macro_rules! macro_10 {
    () => {
        test ! (case : multi_byte_with_new_lines , text : "01\t345\n789abcΔf01234567\u{07}9\nbcΔf" , lines : vec ! [7 , 27] , multi_byte_chars : vec ! [(1 , (6 , 8)) , (2 , (2 , 4))] ,) ;
    };
}

macro_10!()