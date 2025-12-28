macro_rules! deps {
    () => {
        SearchTest!();
    };
}

macro_rules! REGRESSION {
    () => {
        deps!();
        # [doc = " Regression tests that are applied to all Aho-Corasick combinations."] # [doc = ""] # [doc = " If regression tests are needed for specific match semantics, then add them"] # [doc = " to the appropriate group above."] const REGRESSION : & 'static [SearchTest] = & [t ! (regression010 , & ["inf" , "ind"] , "infind" , & [(0 , 0 , 3) , (1 , 3 , 6) ,]) , t ! (regression020 , & ["ind" , "inf"] , "infind" , & [(1 , 0 , 3) , (0 , 3 , 6) ,]) , t ! (regression030 , & ["libcore/" , "libstd/"] , "libcore/char/methods.rs" , & [(0 , 0 , 8) ,]) , t ! (regression040 , & ["libstd/" , "libcore/"] , "libcore/char/methods.rs" , & [(1 , 0 , 8) ,]) , t ! (regression050 , & ["\x00\x00\x01" , "\x00\x00\x00"] , "\x00\x00\x00" , & [(1 , 0 , 3) ,]) , t ! (regression060 , & ["\x00\x00\x00" , "\x00\x00\x01"] , "\x00\x00\x00" , & [(0 , 0 , 3) ,]) ,] ;
    };
}

REGRESSION!()