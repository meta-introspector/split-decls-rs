// Generated macro for impl_446 (impl)
macro_rules! Depcrate_duration_durationimpl_446 {
() => {
// Module: crate::duration::duration
// Provides: {"impl_446"}
// Dependencies: {}
impl Duration { # [doc = " Iterate over the units of the duration in descending order."] pub (crate) fn iter_units (& self) -> [u64 ; 10] { [self . years , self . months , self . weeks , self . days , self . hours , self . minutes , self . seconds , self . milliseconds , self . microseconds , self . nanoseconds ,] } pub (crate) fn get_sign (& self) -> fixed_decimal :: Sign { for & unit in self . iter_units () . iter () { if unit != 0 { return match self . sign { DurationSign :: Positive => fixed_decimal :: Sign :: None , DurationSign :: Negative => fixed_decimal :: Sign :: Negative , } ; } } fixed_decimal :: Sign :: None } }
};
}
