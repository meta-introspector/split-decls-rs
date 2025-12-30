// Generated macro for test_weekdays_iter (function)
macro_rules! Depcrate_weektest_weekdays_iter {
() => {
// Module: crate::week
// Provides: {"test_weekdays_iter"}
// Dependencies: {}
# [test] fn test_weekdays_iter () { use Weekday :: * ; let default_weekend = WeekdaySetIterator :: new (Monday , WeekdaySet :: new (& [Saturday , Sunday])) ; assert_eq ! (vec ! [Saturday , Sunday] , default_weekend . collect ::< Vec < _ >> ()) ; let fri_sun_weekend = WeekdaySetIterator :: new (Monday , WeekdaySet :: new (& [Friday , Sunday])) ; assert_eq ! (vec ! [Friday , Sunday] , fri_sun_weekend . collect ::< Vec < _ >> ()) ; let multiple_contiguous_days = WeekdaySetIterator :: new (Monday , WeekdaySet :: new (& [Weekday :: Tuesday , Weekday :: Wednesday , Weekday :: Thursday , Weekday :: Friday ,]) ,) ; assert_eq ! (vec ! [Tuesday , Wednesday , Thursday , Friday] , multiple_contiguous_days . collect ::< Vec < _ >> ()) ; let multiple_non_contiguous_days = WeekdaySetIterator :: new (Wednesday , WeekdaySet :: new (& [Weekday :: Tuesday , Weekday :: Thursday , Weekday :: Friday , Weekday :: Sunday ,]) ,) ; assert_eq ! (vec ! [Thursday , Friday , Sunday , Tuesday] , multiple_non_contiguous_days . collect ::< Vec < _ >> ()) ; }
};
}
