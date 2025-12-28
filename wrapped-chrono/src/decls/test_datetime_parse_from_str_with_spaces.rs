macro_rules! deps {
    () => {
        NaiveDate!();
        NaiveDateTime!();
    };
}

macro_rules! test_datetime_parse_from_str_with_spaces {
    () => {
        deps!();
        # [test] fn test_datetime_parse_from_str_with_spaces () { let parse_from_str = NaiveDateTime :: parse_from_str ; let dt = NaiveDate :: from_ymd_opt (2013 , 8 , 9) . unwrap () . and_hms_opt (23 , 54 , 35) . unwrap () ; assert_eq ! (parse_from_str (" Aug 09 2013 23:54:35" , " %b %d %Y %H:%M:%S") , Ok (dt)) ; assert_eq ! (parse_from_str ("Aug 09 2013 23:54:35 " , "%b %d %Y %H:%M:%S ") , Ok (dt)) ; assert_eq ! (parse_from_str (" Aug 09 2013  23:54:35 " , " %b %d %Y  %H:%M:%S ") , Ok (dt)) ; assert_eq ! (parse_from_str ("  Aug 09 2013 23:54:35" , "  %b %d %Y %H:%M:%S") , Ok (dt)) ; assert_eq ! (parse_from_str ("   Aug 09 2013 23:54:35" , "   %b %d %Y %H:%M:%S") , Ok (dt)) ; assert_eq ! (parse_from_str ("\n\tAug 09 2013 23:54:35  " , "\n\t%b %d %Y %H:%M:%S  ") , Ok (dt)) ; assert_eq ! (parse_from_str ("\tAug 09 2013 23:54:35\t" , "\t%b %d %Y %H:%M:%S\t") , Ok (dt)) ; assert_eq ! (parse_from_str ("Aug  09 2013 23:54:35" , "%b  %d %Y %H:%M:%S") , Ok (dt)) ; assert_eq ! (parse_from_str ("Aug    09 2013 23:54:35" , "%b    %d %Y %H:%M:%S") , Ok (dt)) ; assert_eq ! (parse_from_str ("Aug  09 2013\t23:54:35" , "%b  %d %Y\t%H:%M:%S") , Ok (dt)) ; assert_eq ! (parse_from_str ("Aug  09 2013\t\t23:54:35" , "%b  %d %Y\t\t%H:%M:%S") , Ok (dt)) ; assert_eq ! (parse_from_str ("Aug 09 2013 23:54:35 " , "%b %d %Y %H:%M:%S\n") , Ok (dt)) ; assert_eq ! (parse_from_str ("Aug 09 2013 23:54:35" , "%b %d %Y\t%H:%M:%S") , Ok (dt)) ; assert_eq ! (parse_from_str ("Aug 09 2013 23:54:35" , "%b %d %Y %H:%M:%S ") , Ok (dt)) ; assert_eq ! (parse_from_str ("Aug 09 2013 23:54:35" , " %b %d %Y %H:%M:%S") , Ok (dt)) ; assert_eq ! (parse_from_str ("Aug 09 2013 23:54:35" , "%b %d %Y %H:%M:%S\n") , Ok (dt)) ; assert ! (parse_from_str (" Aug 09 2013 23:54:35" , "%b %d %Y %H:%M:%S") . is_err ()) ; assert ! (parse_from_str ("Aug 09 2013 23:54:35 " , "%b %d %Y %H:%M:%S") . is_err ()) ; assert ! (parse_from_str ("Aug 09 2013 23:54:35\t" , "%b %d %Y %H:%M:%S") . is_err ()) ; assert ! (parse_from_str ("\nAug 09 2013 23:54:35" , "%b %d %Y %H:%M:%S\n") . is_err ()) ; assert ! (parse_from_str ("Aug 09 2013 23:54:35 !!!" , "%b %d %Y %H:%M:%S ") . is_err ()) ; }
    };
}

test_datetime_parse_from_str_with_spaces!();