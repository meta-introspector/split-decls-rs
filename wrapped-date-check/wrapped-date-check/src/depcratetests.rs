// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_months_since () { let date1 = Date { year : 2020 , month : 3 } ; let date2 = Date { year : 2021 , month : 1 } ; assert_eq ! (date2 . months_since (date1) , Some (10)) ; } # [test] fn test_date_regex () { let regex = & make_date_regex () ; assert ! (regex . is_match ("<!-- date-check: jan 2021 -->")) ; assert ! (regex . is_match ("<!-- date-check: january 2021 -->")) ; assert ! (regex . is_match ("<!-- date-check: Jan 2021 -->")) ; assert ! (regex . is_match ("<!-- date-check: January 2021 -->")) ; assert ! (regex . is_match ("<!-- date-check --> jan 2021")) ; assert ! (regex . is_match ("<!-- date-check --> january 2021")) ; assert ! (regex . is_match ("<!-- date-check --> Jan 2021")) ; assert ! (regex . is_match ("<!-- date-check --> January 2021")) ; assert ! (regex . is_match ("<!-- date-check --> jan 2021 ")) ; assert ! (regex . is_match ("<!-- date-check --> jan 2021.")) ; } # [test] fn test_date_regex_fail () { let regexes = & make_date_regex () ; assert ! (! regexes . is_match ("<!-- date-check: jan 221 -->")) ; assert ! (! regexes . is_match ("<!-- date-check: jan 20221 -->")) ; assert ! (! regexes . is_match ("<!-- date-check: 01 2021 -->")) ; assert ! (! regexes . is_match ("<!-- date-check --> jan 221")) ; assert ! (! regexes . is_match ("<!-- date-check --> jan 20222")) ; assert ! (! regexes . is_match ("<!-- date-check --> 01 2021")) ; } # [test] fn test_collect_dates_from_file () { let text = r"
Test1
<!-- date-check: jan 2021 -->
Test2
Foo<!-- date-check: february 2021
-->
Test3
Test4
Foo<!-- date-check: Mar 2021 -->Bar
<!-- date-check:April 2021
-->
Test5
Test6
Test7
<!-- date-check:

may 2021 -->
Test8
Test1
<!-- date-check -->  jan 2021
Test2
Foo<!-- date-check
--> february 2021
Test3
Test4
Foo<!-- date-check -->  mar 2021 Bar
<!-- date-check
--> apr 2021
Test5
Test6
Test7
<!-- date-check

 --> may 2021
Test8
 <!--
   date-check
 --> june 2021.
        " ; assert_eq ! (collect_dates_from_file (& make_date_regex () , text) , vec ! [(3 , Date { year : 2021 , month : 1 }) , (6 , Date { year : 2021 , month : 2 }) , (9 , Date { year : 2021 , month : 3 }) , (11 , Date { year : 2021 , month : 4 }) , (17 , Date { year : 2021 , month : 5 }) , (20 , Date { year : 2021 , month : 1 }) , (23 , Date { year : 2021 , month : 2 }) , (26 , Date { year : 2021 , month : 3 }) , (28 , Date { year : 2021 , month : 4 }) , (34 , Date { year : 2021 , month : 5 }) , (38 , Date { year : 2021 , month : 6 }) ,] ,) ; } }
};
}
