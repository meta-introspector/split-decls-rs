macro_rules! deps {
    () => {
        NaiveDate!();
    };
}

macro_rules! test_date_pred {
    () => {
        deps!();
        # [test] fn test_date_pred () { let ymd = | y , m , d | NaiveDate :: from_ymd_opt (y , m , d) . unwrap () ; assert_eq ! (ymd (2016 , 3 , 1) . pred_opt () , Some (ymd (2016 , 2 , 29))) ; assert_eq ! (ymd (2015 , 1 , 1) . pred_opt () , Some (ymd (2014 , 12 , 31))) ; assert_eq ! (ymd (2014 , 6 , 1) . pred_opt () , Some (ymd (2014 , 5 , 31))) ; assert_eq ! (ymd (2014 , 5 , 7) . pred_opt () , Some (ymd (2014 , 5 , 6))) ; assert_eq ! (ymd (NaiveDate :: MIN . year () , 1 , 1) . pred_opt () , None) ; }
    };
}

test_date_pred!();