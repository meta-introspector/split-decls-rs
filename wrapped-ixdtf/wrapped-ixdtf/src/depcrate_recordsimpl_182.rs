// Generated macro for impl_182 (impl)
macro_rules! Depcrate_recordsimpl_182 {
() => {
// Module: crate::records
// Provides: {"impl_182"}
// Dependencies: {}
impl UtcOffsetRecord { # [doc = " Returns whether the UTC offset is a minute precision offset."] pub fn is_minute_precision (& self) -> bool { matches ! (self , Self :: MinutePrecision (_)) } # [doc = " Returrns a zerod UTC Offset in minute precision"] pub fn zero () -> Self { Self :: MinutePrecision (MinutePrecisionOffset :: zero ()) } # [doc = " Returns the `Sign` of this UTC offset."] pub fn sign (& self) -> Sign { match self { Self :: MinutePrecision (offset) => offset . sign , Self :: FullPrecisionOffset (offset) => offset . minute_precision_offset . sign , } } # [doc = " Returns the hour value of this UTC offset."] pub fn hour (& self) -> u8 { match self { Self :: MinutePrecision (offset) => offset . hour , Self :: FullPrecisionOffset (offset) => offset . minute_precision_offset . hour , } } # [doc = " Returns the minute value of this UTC offset."] pub fn minute (& self) -> u8 { match self { Self :: MinutePrecision (offset) => offset . minute , Self :: FullPrecisionOffset (offset) => offset . minute_precision_offset . minute , } } # [doc = " Returns the second value of this UTC offset if it is a full precision offset."] pub fn second (& self) -> Option < u8 > { match self { Self :: MinutePrecision (_) => None , Self :: FullPrecisionOffset (offset) => Some (offset . second) , } } # [doc = " Returns the fraction value of this UTC offset if it is a full precision offset."] pub fn fraction (& self) -> Option < Fraction > { match self { Self :: MinutePrecision (_) => None , Self :: FullPrecisionOffset (offset) => offset . fraction , } } }
};
}
