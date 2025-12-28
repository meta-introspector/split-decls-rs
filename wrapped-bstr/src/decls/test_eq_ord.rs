macro_rules! deps {
    () => {
        BString!();
        BStr!();
    };
}

macro_rules! test_eq_ord {
    () => {
        deps!();
        # [test] # [cfg (feature = "alloc")] fn test_eq_ord () { use core :: cmp :: Ordering ; use crate :: { BStr , BString } ; let b = BStr :: new ("hello") ; assert_eq ! (b , b"hello") ; assert_ne ! (b , b"world") ; assert_eq ! (b . partial_cmp (b"hello") , Some (Ordering :: Equal)) ; assert_eq ! (b . partial_cmp (b"world") , Some (Ordering :: Less)) ; let b = BString :: from ("hello") ; assert_eq ! (b , b"hello") ; assert_ne ! (b , b"world") ; assert_eq ! (b . partial_cmp (b"hello") , Some (Ordering :: Equal)) ; assert_eq ! (b . partial_cmp (b"world") , Some (Ordering :: Less)) ; }
    };
}

test_eq_ord!()