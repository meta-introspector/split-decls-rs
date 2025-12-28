macro_rules! macro_3 {
    () => {
        test ! (case : newlines_long , text : "012345678\nabcdef012345678\na" , lines : vec ! [10 , 26] , multi_byte_chars : vec ! [] ,) ;
    };
}

macro_3!();