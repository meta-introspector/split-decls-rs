macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! test_inline {
    () => {
        deps!();
        # [test] pub fn test_inline () { let mut v = SmallVec :: < _ , 16 > :: new () ; v . push ("hello" . to_owned ()) ; v . push ("there" . to_owned ()) ; assert_eq ! (&* v , & ["hello" . to_owned () , "there" . to_owned () ,] [..]) ; }
    };
}

test_inline!()