macro_rules! deps {
    () => {
        NaiveDate!();
    };
}

macro_rules! test_date_from_yo {
    () => {
        deps!();
        # [test] fn test_date_from_yo () { let from_yo = NaiveDate :: from_yo_opt ; let ymd = | y , m , d | NaiveDate :: from_ymd_opt (y , m , d) . unwrap () ; assert_eq ! (from_yo (2012 , 0) , None) ; assert_eq ! (from_yo (2012 , 1) , Some (ymd (2012 , 1 , 1))) ; assert_eq ! (from_yo (2012 , 2) , Some (ymd (2012 , 1 , 2))) ; assert_eq ! (from_yo (2012 , 32) , Some (ymd (2012 , 2 , 1))) ; assert_eq ! (from_yo (2012 , 60) , Some (ymd (2012 , 2 , 29))) ; assert_eq ! (from_yo (2012 , 61) , Some (ymd (2012 , 3 , 1))) ; assert_eq ! (from_yo (2012 , 100) , Some (ymd (2012 , 4 , 9))) ; assert_eq ! (from_yo (2012 , 200) , Some (ymd (2012 , 7 , 18))) ; assert_eq ! (from_yo (2012 , 300) , Some (ymd (2012 , 10 , 26))) ; assert_eq ! (from_yo (2012 , 366) , Some (ymd (2012 , 12 , 31))) ; assert_eq ! (from_yo (2012 , 367) , None) ; assert_eq ! (from_yo (2012 , (1 << 28) | 60) , None) ; assert_eq ! (from_yo (2014 , 0) , None) ; assert_eq ! (from_yo (2014 , 1) , Some (ymd (2014 , 1 , 1))) ; assert_eq ! (from_yo (2014 , 2) , Some (ymd (2014 , 1 , 2))) ; assert_eq ! (from_yo (2014 , 32) , Some (ymd (2014 , 2 , 1))) ; assert_eq ! (from_yo (2014 , 59) , Some (ymd (2014 , 2 , 28))) ; assert_eq ! (from_yo (2014 , 60) , Some (ymd (2014 , 3 , 1))) ; assert_eq ! (from_yo (2014 , 100) , Some (ymd (2014 , 4 , 10))) ; assert_eq ! (from_yo (2014 , 200) , Some (ymd (2014 , 7 , 19))) ; assert_eq ! (from_yo (2014 , 300) , Some (ymd (2014 , 10 , 27))) ; assert_eq ! (from_yo (2014 , 365) , Some (ymd (2014 , 12 , 31))) ; assert_eq ! (from_yo (2014 , 366) , None) ; }
    };
}

test_date_from_yo!();