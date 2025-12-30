// Generated macro for test_weekdayset_new (function)
macro_rules! Depcrate_providertest_weekdayset_new {
() => {
// Module: crate::provider
// Provides: {"test_weekdayset_new"}
// Dependencies: {}
# [test] fn test_weekdayset_new () { use Weekday :: * ; let sat_sun_bitmap = Saturday . bit_value () | Sunday . bit_value () ; let sat_sun_weekend = WeekdaySet :: new (& [Saturday , Sunday]) ; assert_eq ! (sat_sun_bitmap , sat_sun_weekend . 0) ; let fri_sat_bitmap = Friday . bit_value () | Saturday . bit_value () ; let fri_sat_weekend = WeekdaySet :: new (& [Friday , Saturday]) ; assert_eq ! (fri_sat_bitmap , fri_sat_weekend . 0) ; let fri_sun_bitmap = Friday . bit_value () | Sunday . bit_value () ; let fri_sun_weekend = WeekdaySet :: new (& [Friday , Sunday]) ; assert_eq ! (fri_sun_bitmap , fri_sun_weekend . 0) ; let fri_bitmap = Friday . bit_value () ; let fri_weekend = WeekdaySet :: new (& [Friday , Friday]) ; assert_eq ! (fri_bitmap , fri_weekend . 0) ; let sun_mon_bitmap = Sunday . bit_value () | Monday . bit_value () ; let sun_mon_weekend = WeekdaySet :: new (& [Sunday , Monday]) ; assert_eq ! (sun_mon_bitmap , sun_mon_weekend . 0) ; let mon_sun_bitmap = Monday . bit_value () | Sunday . bit_value () ; let mon_sun_weekend = WeekdaySet :: new (& [Monday , Sunday]) ; assert_eq ! (mon_sun_bitmap , mon_sun_weekend . 0) ; let mon_bitmap = Monday . bit_value () ; let mon_weekend = WeekdaySet :: new (& [Monday]) ; assert_eq ! (mon_bitmap , mon_weekend . 0) ; }
};
}
