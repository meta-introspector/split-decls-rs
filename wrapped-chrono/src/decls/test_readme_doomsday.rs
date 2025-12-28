macro_rules! deps {
    () => {
        NaiveDate!();
    };
}

macro_rules! test_readme_doomsday {
    () => {
        deps!();
        # [test] fn test_readme_doomsday () { for y in NaiveDate :: MIN . year () ..= NaiveDate :: MAX . year () { let d4 = NaiveDate :: from_ymd_opt (y , 4 , 4) . unwrap () ; let d6 = NaiveDate :: from_ymd_opt (y , 6 , 6) . unwrap () ; let d8 = NaiveDate :: from_ymd_opt (y , 8 , 8) . unwrap () ; let d10 = NaiveDate :: from_ymd_opt (y , 10 , 10) . unwrap () ; let d12 = NaiveDate :: from_ymd_opt (y , 12 , 12) . unwrap () ; let d59 = NaiveDate :: from_ymd_opt (y , 5 , 9) . unwrap () ; let d95 = NaiveDate :: from_ymd_opt (y , 9 , 5) . unwrap () ; let d711 = NaiveDate :: from_ymd_opt (y , 7 , 11) . unwrap () ; let d117 = NaiveDate :: from_ymd_opt (y , 11 , 7) . unwrap () ; let d30 = NaiveDate :: from_ymd_opt (y , 3 , 1) . unwrap () . pred_opt () . unwrap () ; let weekday = d30 . weekday () ; let other_dates = [d4 , d6 , d8 , d10 , d12 , d59 , d95 , d711 , d117] ; assert ! (other_dates . iter () . all (| d | d . weekday () == weekday)) ; } }
    };
}

test_readme_doomsday!()