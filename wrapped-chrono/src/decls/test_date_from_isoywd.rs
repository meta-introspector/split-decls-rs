macro_rules! deps {
    () => {
        NaiveDate!();
        Weekday!();
    };
}

macro_rules! test_date_from_isoywd {
    () => {
        deps!();
        # [test] fn test_date_from_isoywd () { let from_isoywd = NaiveDate :: from_isoywd_opt ; let ymd = | y , m , d | NaiveDate :: from_ymd_opt (y , m , d) . unwrap () ; assert_eq ! (from_isoywd (2004 , 0 , Weekday :: Sun) , None) ; assert_eq ! (from_isoywd (2004 , 1 , Weekday :: Mon) , Some (ymd (2003 , 12 , 29))) ; assert_eq ! (from_isoywd (2004 , 1 , Weekday :: Sun) , Some (ymd (2004 , 1 , 4))) ; assert_eq ! (from_isoywd (2004 , 2 , Weekday :: Mon) , Some (ymd (2004 , 1 , 5))) ; assert_eq ! (from_isoywd (2004 , 2 , Weekday :: Sun) , Some (ymd (2004 , 1 , 11))) ; assert_eq ! (from_isoywd (2004 , 52 , Weekday :: Mon) , Some (ymd (2004 , 12 , 20))) ; assert_eq ! (from_isoywd (2004 , 52 , Weekday :: Sun) , Some (ymd (2004 , 12 , 26))) ; assert_eq ! (from_isoywd (2004 , 53 , Weekday :: Mon) , Some (ymd (2004 , 12 , 27))) ; assert_eq ! (from_isoywd (2004 , 53 , Weekday :: Sun) , Some (ymd (2005 , 1 , 2))) ; assert_eq ! (from_isoywd (2004 , 54 , Weekday :: Mon) , None) ; assert_eq ! (from_isoywd (2011 , 0 , Weekday :: Sun) , None) ; assert_eq ! (from_isoywd (2011 , 1 , Weekday :: Mon) , Some (ymd (2011 , 1 , 3))) ; assert_eq ! (from_isoywd (2011 , 1 , Weekday :: Sun) , Some (ymd (2011 , 1 , 9))) ; assert_eq ! (from_isoywd (2011 , 2 , Weekday :: Mon) , Some (ymd (2011 , 1 , 10))) ; assert_eq ! (from_isoywd (2011 , 2 , Weekday :: Sun) , Some (ymd (2011 , 1 , 16))) ; assert_eq ! (from_isoywd (2018 , 51 , Weekday :: Mon) , Some (ymd (2018 , 12 , 17))) ; assert_eq ! (from_isoywd (2018 , 51 , Weekday :: Sun) , Some (ymd (2018 , 12 , 23))) ; assert_eq ! (from_isoywd (2018 , 52 , Weekday :: Mon) , Some (ymd (2018 , 12 , 24))) ; assert_eq ! (from_isoywd (2018 , 52 , Weekday :: Sun) , Some (ymd (2018 , 12 , 30))) ; assert_eq ! (from_isoywd (2018 , 53 , Weekday :: Mon) , None) ; }
    };
}

test_date_from_isoywd!();