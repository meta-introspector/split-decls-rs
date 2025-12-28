macro_rules! deps {
    () => {
        SearchTest!();
    };
}

macro_rules! ASCII_CASE_INSENSITIVE_OVERLAPPING {
    () => {
        deps!();
        # [doc = " Like ASCII_CASE_INSENSITIVE, but specifically for overlapping tests."] const ASCII_CASE_INSENSITIVE_OVERLAPPING : & 'static [SearchTest] = & [t ! (acasei000 , & ["foo" , "FOO"] , "fOo" , & [(0 , 0 , 3) , (1 , 0 , 3)]) , t ! (acasei001 , & ["FOO" , "foo"] , "fOo" , & [(0 , 0 , 3) , (1 , 0 , 3)]) , t ! (acasei010 , & ["abc" , "def" , "abcdef"] , "abcdef" , & [(0 , 0 , 3) , (2 , 0 , 6) , (1 , 3 , 6)]) ,] ;
    };
}

ASCII_CASE_INSENSITIVE_OVERLAPPING!();