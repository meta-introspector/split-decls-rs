macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! test_spill {
    () => {
        deps!();
        # [test] pub fn test_spill () { let mut v = SmallVec :: < _ , 2 > :: new () ; v . push ("hello" . to_owned ()) ; assert_eq ! (v [0] , "hello") ; v . push ("there" . to_owned ()) ; v . push ("burma" . to_owned ()) ; assert_eq ! (v [0] , "hello") ; v . push ("shave" . to_owned ()) ; assert_eq ! (&* v , & ["hello" . to_owned () , "there" . to_owned () , "burma" . to_owned () , "shave" . to_owned () ,] [..]) ; }
    };
}

test_spill!();