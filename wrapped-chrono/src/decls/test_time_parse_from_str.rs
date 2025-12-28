macro_rules! deps {
    () => {
        NaiveTime!();
    };
}

macro_rules! test_time_parse_from_str {
    () => {
        deps!();
        # [test] fn test_time_parse_from_str () { let hms = | h , m , s | NaiveTime :: from_hms_opt (h , m , s) . unwrap () ; assert_eq ! (NaiveTime :: parse_from_str ("2014-5-7T12:34:56+09:30" , "%Y-%m-%dT%H:%M:%S%z") , Ok (hms (12 , 34 , 56))) ; assert_eq ! (NaiveTime :: parse_from_str ("PM 12:59" , "%P %H:%M") , Ok (hms (12 , 59 , 0))) ; assert_eq ! (NaiveTime :: parse_from_str ("12:59 \n\t PM" , "%H:%M \n\t %P") , Ok (hms (12 , 59 , 0))) ; assert_eq ! (NaiveTime :: parse_from_str ("\t\t12:59\tPM\t" , "\t\t%H:%M\t%P\t") , Ok (hms (12 , 59 , 0))) ; assert_eq ! (NaiveTime :: parse_from_str ("\t\t1259\t\tPM\t" , "\t\t%H%M\t\t%P\t") , Ok (hms (12 , 59 , 0))) ; assert ! (NaiveTime :: parse_from_str ("12:59 PM" , "%H:%M\t%P") . is_ok ()) ; assert ! (NaiveTime :: parse_from_str ("\t\t12:59 PM\t" , "\t\t%H:%M\t%P\t") . is_ok ()) ; assert ! (NaiveTime :: parse_from_str ("12:59  PM" , "%H:%M %P") . is_ok ()) ; assert ! (NaiveTime :: parse_from_str ("12:3456" , "%H:%M:%S") . is_err ()) ; }
    };
}

test_time_parse_from_str!();