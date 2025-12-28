macro_rules! deps {
    () => {
        NaiveDate!();
        TimeDelta!();
    };
}

macro_rules! test_datetime_add {
    () => {
        deps!();
        # [test] fn test_datetime_add () { fn check ((y , m , d , h , n , s) : (i32 , u32 , u32 , u32 , u32 , u32) , rhs : TimeDelta , result : Option < (i32 , u32 , u32 , u32 , u32 , u32) > ,) { let lhs = NaiveDate :: from_ymd_opt (y , m , d) . unwrap () . and_hms_opt (h , n , s) . unwrap () ; let sum = result . map (| (y , m , d , h , n , s) | { NaiveDate :: from_ymd_opt (y , m , d) . unwrap () . and_hms_opt (h , n , s) . unwrap () }) ; assert_eq ! (lhs . checked_add_signed (rhs) , sum) ; assert_eq ! (lhs . checked_sub_signed (- rhs) , sum) ; } let seconds = | s | TimeDelta :: try_seconds (s) . unwrap () ; check ((2014 , 5 , 6 , 7 , 8 , 9) , seconds (3600 + 60 + 1) , Some ((2014 , 5 , 6 , 8 , 9 , 10))) ; check ((2014 , 5 , 6 , 7 , 8 , 9) , seconds (- (3600 + 60 + 1)) , Some ((2014 , 5 , 6 , 6 , 7 , 8))) ; check ((2014 , 5 , 6 , 7 , 8 , 9) , seconds (86399) , Some ((2014 , 5 , 7 , 7 , 8 , 8))) ; check ((2014 , 5 , 6 , 7 , 8 , 9) , seconds (86_400 * 10) , Some ((2014 , 5 , 16 , 7 , 8 , 9))) ; check ((2014 , 5 , 6 , 7 , 8 , 9) , seconds (- 86_400 * 10) , Some ((2014 , 4 , 26 , 7 , 8 , 9))) ; check ((2014 , 5 , 6 , 7 , 8 , 9) , seconds (86_400 * 10) , Some ((2014 , 5 , 16 , 7 , 8 , 9))) ; let max_days_from_year_0 = NaiveDate :: MAX . signed_duration_since (NaiveDate :: from_ymd_opt (0 , 1 , 1) . unwrap ()) ; check ((0 , 1 , 1 , 0 , 0 , 0) , max_days_from_year_0 , Some ((NaiveDate :: MAX . year () , 12 , 31 , 0 , 0 , 0))) ; check ((0 , 1 , 1 , 0 , 0 , 0) , max_days_from_year_0 + seconds (86399) , Some ((NaiveDate :: MAX . year () , 12 , 31 , 23 , 59 , 59)) ,) ; check ((0 , 1 , 1 , 0 , 0 , 0) , max_days_from_year_0 + seconds (86_400) , None) ; check ((0 , 1 , 1 , 0 , 0 , 0) , TimeDelta :: MAX , None) ; let min_days_from_year_0 = NaiveDate :: MIN . signed_duration_since (NaiveDate :: from_ymd_opt (0 , 1 , 1) . unwrap ()) ; check ((0 , 1 , 1 , 0 , 0 , 0) , min_days_from_year_0 , Some ((NaiveDate :: MIN . year () , 1 , 1 , 0 , 0 , 0))) ; check ((0 , 1 , 1 , 0 , 0 , 0) , min_days_from_year_0 - seconds (1) , None) ; check ((0 , 1 , 1 , 0 , 0 , 0) , TimeDelta :: MIN , None) ; }
    };
}

test_datetime_add!();