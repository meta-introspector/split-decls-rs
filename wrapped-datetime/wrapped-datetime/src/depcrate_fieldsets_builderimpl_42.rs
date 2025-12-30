// Generated macro for impl_42 (impl)
macro_rules! Depcrate_fieldsets_builderimpl_42 {
() => {
// Module: crate::fieldsets::builder
// Provides: {"impl_42"}
// Dependencies: {}
impl DateFields { # [doc = " All values of this enumeration."] pub const VALUES : & [Self] = & [Self :: D , Self :: MD , Self :: YMD , Self :: DE , Self :: MDE , Self :: YMDE , Self :: E , Self :: M , Self :: YM , Self :: Y ,] ; # [doc = " Returns whether this [`DateFields`] variant represents a [`CalendarPeriodFieldSet`]."] pub fn is_calendar_period (self) -> bool { match self { DateFields :: D => false , DateFields :: MD => false , DateFields :: YMD => false , DateFields :: DE => false , DateFields :: MDE => false , DateFields :: YMDE => false , DateFields :: E => false , DateFields :: M => true , DateFields :: YM => true , DateFields :: Y => true , } } }
};
}
