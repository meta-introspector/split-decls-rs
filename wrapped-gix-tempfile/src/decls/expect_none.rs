macro_rules! expect_none {
    () => {
        fn expect_none < T > (v : Option < T >) { assert ! (v . is_none () , "there should never be conflicts or old values as ids are never reused.") ; }
    };
}

expect_none!();