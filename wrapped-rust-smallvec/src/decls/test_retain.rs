macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! test_retain {
    () => {
        deps!();
        # [test] fn test_retain () { let mut sv : SmallVec < i32 , 5 > = SmallVec :: from (& [1 , 2 , 3 , 3 , 4]) ; sv . retain (| & i | i != 3) ; assert_eq ! (sv . pop () , Some (4)) ; assert_eq ! (sv . pop () , Some (2)) ; assert_eq ! (sv . pop () , Some (1)) ; assert_eq ! (sv . pop () , None) ; let mut sv : SmallVec < i32 , 3 > = SmallVec :: from (& [1 , 2 , 3 , 3 , 4]) ; sv . retain (| & i | i != 3) ; assert_eq ! (sv . pop () , Some (4)) ; assert_eq ! (sv . pop () , Some (2)) ; assert_eq ! (sv . pop () , Some (1)) ; assert_eq ! (sv . pop () , None) ; let one = Rc :: new (1) ; let mut sv : SmallVec < Rc < i32 > , 3 > = SmallVec :: new () ; sv . push (Rc :: clone (& one)) ; assert_eq ! (Rc :: strong_count (& one) , 2) ; sv . retain (| _ | false) ; assert_eq ! (Rc :: strong_count (& one) , 1) ; let mut sv : SmallVec < Rc < i32 > , 1 > = SmallVec :: new () ; sv . push (Rc :: clone (& one)) ; sv . push (Rc :: new (2)) ; assert_eq ! (Rc :: strong_count (& one) , 2) ; sv . retain (| _ | false) ; assert_eq ! (Rc :: strong_count (& one) , 1) ; }
    };
}

test_retain!()