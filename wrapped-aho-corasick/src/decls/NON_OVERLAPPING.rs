macro_rules! deps {
    () => {
        SearchTest!();
    };
}

macro_rules! NON_OVERLAPPING {
    () => {
        deps!();
        # [doc = " Tests for non-overlapping match semantics."] # [doc = ""] # [doc = " Generally these tests shouldn't pass when using overlapping semantics."] # [doc = " These should pass for both standard and leftmost match semantics."] const NON_OVERLAPPING : & 'static [SearchTest] = & [t ! (nover010 , & ["abcd" , "bcd" , "cd"] , "abcd" , & [(0 , 0 , 4) ,]) , t ! (nover020 , & ["bcd" , "cd" , "abcd"] , "abcd" , & [(2 , 0 , 4) ,]) , t ! (nover030 , & ["abc" , "bc"] , "zazabcz" , & [(0 , 3 , 6) ,]) , t ! (nover100 , & ["ab" , "ba"] , "abababa" , & [(0 , 0 , 2) , (0 , 2 , 4) , (0 , 4 , 6) ,]) , t ! (nover200 , & ["foo" , "foo"] , "foobarfoo" , & [(0 , 0 , 3) , (0 , 6 , 9) ,]) , t ! (nover300 , & ["" , ""] , "" , & [(0 , 0 , 0) ,]) , t ! (nover310 , & ["" , ""] , "a" , & [(0 , 0 , 0) , (0 , 1 , 1) ,]) ,] ;
    };
}

NON_OVERLAPPING!()