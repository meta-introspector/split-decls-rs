// Generated macro for ConvertCalendar (trait)
macro_rules! Depcrate_scaffold_calendarConvertCalendar {
() => {
// Module: crate::scaffold::calendar
// Provides: {"ConvertCalendar"}
// Dependencies: {}
# [doc = " A type that can be converted into a specific calendar system."] pub trait ConvertCalendar { # [doc = " The converted type. This can be the same as the receiver type."] type Converted < 'a > : Sized ; # [doc = " Converts `self` to the specified [`AnyCalendar`]."] fn to_calendar < 'a > (& self , calendar : & 'a AnyCalendar) -> Self :: Converted < 'a > ; }
};
}
