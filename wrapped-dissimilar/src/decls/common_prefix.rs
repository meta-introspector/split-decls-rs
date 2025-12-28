macro_rules! common_prefix {
    () => {
        fn common_prefix (text1 : Range , text2 : Range) -> usize { for (i , (b1 , b2)) in text1 . chars () . zip (text2 . chars ()) . enumerate () { if b1 != b2 { return i ; } } cmp :: min (text1 . len , text2 . len) }
    };
}

common_prefix!()