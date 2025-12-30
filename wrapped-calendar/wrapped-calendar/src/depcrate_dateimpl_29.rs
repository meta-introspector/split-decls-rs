// Generated macro for impl_29 (impl)
macro_rules! Depcrate_dateimpl_29 {
() => {
// Module: crate::date
// Provides: {"impl_29"}
// Dependencies: {}
impl Date < Iso > { # [doc = " The ISO week of the year containing this date."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use icu::calendar::types::IsoWeekOfYear;"] # [doc = " use icu::calendar::Date;"] # [doc = ""] # [doc = " let date = Date::try_new_iso(2022, 8, 26).unwrap();"] # [doc = ""] # [doc = " assert_eq!("] # [doc = "     date.week_of_year(),"] # [doc = "     IsoWeekOfYear {"] # [doc = "         week_number: 34,"] # [doc = "         iso_year: 2022,"] # [doc = "     }"] # [doc = " );"] # [doc = " ```"] pub fn week_of_year (& self) -> IsoWeekOfYear { let week_of = WeekCalculator :: ISO . week_of (365 + calendrical_calculations :: gregorian :: is_leap_year (self . inner . 0 . year () - 1) as u16 , self . days_in_year () , self . day_of_year () . 0 , self . day_of_week () ,) . unwrap_or_else (| _ | { debug_assert ! (false) ; WeekOf { week : 1 , unit : crate :: week :: RelativeUnit :: Current , } }) ; IsoWeekOfYear { week_number : week_of . week , iso_year : match week_of . unit { RelativeUnit :: Current => self . inner . 0 . year () , RelativeUnit :: Next => self . inner . 0 . year () + 1 , RelativeUnit :: Previous => self . inner . 0 . year () - 1 , } , } } }
};
}
