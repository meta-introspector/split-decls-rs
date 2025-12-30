// Generated macro for impl_420 (impl)
macro_rules! Depcrate_weekimpl_420 {
() => {
// Module: crate::week
// Provides: {"impl_420"}
// Dependencies: {}
impl WeekInformation { icu_provider :: gen_buffer_data_constructors ! ((prefs : WeekPreferences) -> error : DataError , # [doc = " Creates a new [`WeekCalculator`] from compiled data."]) ; # [doc = icu_provider :: gen_buffer_unstable_docs ! (UNSTABLE , Self :: try_new)] pub fn try_new_unstable < P > (provider : & P , prefs : WeekPreferences) -> Result < Self , DataError > where P : DataProvider < crate :: provider :: CalendarWeekV1 > + ? Sized , { let locale = CalendarWeekV1 :: make_locale (prefs . locale_preferences) ; provider . load (DataRequest { id : DataIdentifierBorrowed :: for_locale (& locale) , .. Default :: default () }) . map (| response | WeekInformation { first_weekday : match prefs . first_weekday { Some (FirstDay :: Mon) => Weekday :: Monday , Some (FirstDay :: Tue) => Weekday :: Tuesday , Some (FirstDay :: Wed) => Weekday :: Wednesday , Some (FirstDay :: Thu) => Weekday :: Thursday , Some (FirstDay :: Fri) => Weekday :: Friday , Some (FirstDay :: Sat) => Weekday :: Saturday , Some (FirstDay :: Sun) => Weekday :: Sunday , _ => response . payload . get () . first_weekday , } , weekend : response . payload . get () . weekend , }) } # [doc = " Weekdays that are part of the 'weekend', for calendar purposes."] # [doc = " Days may not be contiguous, and order is based off the first weekday."] pub fn weekend (self) -> WeekdaySetIterator { WeekdaySetIterator :: new (self . first_weekday , self . weekend) } }
};
}
