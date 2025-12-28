macro_rules! deps {
    () => {
        SearchTest!();
    };
}

macro_rules! ANCHORED_NON_OVERLAPPING {
    () => {
        deps!();
        # [doc = " Like NON_OVERLAPPING, but for anchored searches."] const ANCHORED_NON_OVERLAPPING : & 'static [SearchTest] = & [t ! (anover010 , & ["abcd" , "bcd" , "cd"] , "abcd" , & [(0 , 0 , 4) ,]) , t ! (anover020 , & ["bcd" , "cd" , "abcd"] , "abcd" , & [(2 , 0 , 4) ,]) , t ! (anover030 , & ["abc" , "bc"] , "zazabcz" , & []) , t ! (anover100 , & ["ab" , "ba"] , "abababa" , & [(0 , 0 , 2) , (0 , 2 , 4) , (0 , 4 , 6)]) , t ! (anover200 , & ["foo" , "foo"] , "foobarfoo" , & [(0 , 0 , 3)]) , t ! (anover300 , & ["" , ""] , "" , & [(0 , 0 , 0)]) , t ! (anover310 , & ["" , ""] , "a" , & [(0 , 0 , 0) , (0 , 1 , 1)]) ,] ;
    };
}

ANCHORED_NON_OVERLAPPING!()