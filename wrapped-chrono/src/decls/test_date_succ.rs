macro_rules! deps {
    () => {
        NaiveDate!();
    };
}

macro_rules! test_date_succ {
    () => {
        deps!();
        # [test] fn test_date_succ () { let ymd = | y , m , d | NaiveDate :: from_ymd_opt (y , m , d) . unwrap () ; assert_eq ! (ymd (2014 , 5 , 6) . succ_opt () , Some (ymd (2014 , 5 , 7))) ; assert_eq ! (ymd (2014 , 5 , 31) . succ_opt () , Some (ymd (2014 , 6 , 1))) ; assert_eq ! (ymd (2014 , 12 , 31) . succ_opt () , Some (ymd (2015 , 1 , 1))) ; assert_eq ! (ymd (2016 , 2 , 28) . succ_opt () , Some (ymd (2016 , 2 , 29))) ; assert_eq ! (ymd (NaiveDate :: MAX . year () , 12 , 31) . succ_opt () , None) ; }
    };
}

test_date_succ!()