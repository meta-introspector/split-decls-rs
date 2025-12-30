// Generated macro for impl_732 (impl)
macro_rules! Depcrate_provider_patternimpl_732 {
() => {
// Module: crate::provider::pattern
// Provides: {"impl_732"}
// Dependencies: {}
impl TimeGranularity { # [doc = " Returns [`true`] if the most granular time being displayed will align with"] # [doc = " the top of the hour, otherwise returns [`false`]."] # [doc = " e.g. `12:00:00` is at the top of the hour for any display granularity."] # [doc = " e.g. `12:00:05` is only at the top of the hour if the seconds are not displayed."] pub fn is_top_of_hour (self , minute : u8 , second : u8 , subsecond : u32) -> bool { match self { Self :: None | Self :: Hours => true , Self :: Minutes => minute == 0 , Self :: Seconds => minute == 0 && second == 0 , Self :: Nanoseconds => minute == 0 && second == 0 && subsecond == 0 , } } # [inline] pub (crate) fn from_ordinal (ordinal : u8) -> TimeGranularity { use TimeGranularity :: * ; match ordinal { 1 => Hours , 2 => Minutes , 3 => Seconds , 4 => Nanoseconds , _ => None , } } # [inline] pub (crate) const fn ordinal (self) -> u8 { use TimeGranularity :: * ; match self { None => 0 , Hours => 1 , Minutes => 2 , Seconds => 3 , Nanoseconds => 4 , } } }
};
}
