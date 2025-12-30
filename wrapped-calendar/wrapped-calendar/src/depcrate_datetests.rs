// Generated macro for tests (module)
macro_rules! Depcrate_datetests {
() => {
// Module: crate::date
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use crate :: types :: Weekday ; # [test] fn test_ord () { let dates_in_order = [Date :: try_new_iso (- 10 , 1 , 1) . unwrap () , Date :: try_new_iso (- 10 , 1 , 2) . unwrap () , Date :: try_new_iso (- 10 , 2 , 1) . unwrap () , Date :: try_new_iso (- 1 , 1 , 1) . unwrap () , Date :: try_new_iso (- 1 , 1 , 2) . unwrap () , Date :: try_new_iso (- 1 , 2 , 1) . unwrap () , Date :: try_new_iso (0 , 1 , 1) . unwrap () , Date :: try_new_iso (0 , 1 , 2) . unwrap () , Date :: try_new_iso (0 , 2 , 1) . unwrap () , Date :: try_new_iso (1 , 1 , 1) . unwrap () , Date :: try_new_iso (1 , 1 , 2) . unwrap () , Date :: try_new_iso (1 , 2 , 1) . unwrap () , Date :: try_new_iso (10 , 1 , 1) . unwrap () , Date :: try_new_iso (10 , 1 , 2) . unwrap () , Date :: try_new_iso (10 , 2 , 1) . unwrap () ,] ; for (i , i_date) in dates_in_order . iter () . enumerate () { for (j , j_date) in dates_in_order . iter () . enumerate () { let result1 = i_date . cmp (j_date) ; let result2 = j_date . cmp (i_date) ; assert_eq ! (result1 . reverse () , result2) ; assert_eq ! (i . cmp (& j) , i_date . cmp (j_date)) ; } } } # [test] fn test_day_of_week () { assert_eq ! (Date :: try_new_iso (2021 , 6 , 23) . unwrap () . day_of_week () , Weekday :: Wednesday ,) ; assert_eq ! (Date :: try_new_iso (1983 , 2 , 2) . unwrap () . day_of_week () , Weekday :: Wednesday ,) ; assert_eq ! (Date :: try_new_iso (2020 , 1 , 21) . unwrap () . day_of_week () , Weekday :: Tuesday ,) ; } }
};
}
