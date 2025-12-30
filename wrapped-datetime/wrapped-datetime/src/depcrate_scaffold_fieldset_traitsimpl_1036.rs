// Generated macro for impl_1036 (impl)
macro_rules! Depcrate_scaffold_fieldset_traitsimpl_1036 {
() => {
// Module: crate::scaffold::fieldset_traits
// Provides: {"impl_1036"}
// Dependencies: {}
impl < T , R > AllInputMarkers < R > for T where R : DateTimeMarkers , R :: D : DateInputMarkers , R :: T : TimeMarkers , R :: Z : ZoneMarkers , T : GetField < < R :: D as DateInputMarkers > :: YearInput > + GetField < < R :: D as DateInputMarkers > :: MonthInput > + GetField < < R :: D as DateInputMarkers > :: DayOfMonthInput > + GetField < < R :: D as DateInputMarkers > :: DayOfWeekInput > + GetField < < R :: D as DateInputMarkers > :: DayOfYearInput > + GetField < < R :: D as DateInputMarkers > :: RataDieInput > + GetField < < R :: T as TimeMarkers > :: HourInput > + GetField < < R :: T as TimeMarkers > :: MinuteInput > + GetField < < R :: T as TimeMarkers > :: SecondInput > + GetField < < R :: T as TimeMarkers > :: NanosecondInput > + GetField < < R :: Z as ZoneMarkers > :: TimeZoneIdInput > + GetField < < R :: Z as ZoneMarkers > :: TimeZoneOffsetInput > + GetField < < R :: Z as ZoneMarkers > :: TimeZoneNameTimestampInput > , { }
};
}
