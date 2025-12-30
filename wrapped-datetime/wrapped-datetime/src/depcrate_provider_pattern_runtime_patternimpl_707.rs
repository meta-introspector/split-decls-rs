// Generated macro for impl_707 (impl)
macro_rules! Depcrate_provider_pattern_runtime_patternimpl_707 {
() => {
// Module: crate::provider::pattern::runtime::pattern
// Provides: {"impl_707"}
// Dependencies: {}
impl PatternMetadata { pub (crate) const DEFAULT : PatternMetadata = Self :: from_time_granularity (TimeGranularity :: None) ; # [inline] pub (crate) fn time_granularity (self) -> TimeGranularity { TimeGranularity :: from_ordinal (self . 0) } pub (crate) fn from_items (items : & [PatternItem]) -> Self { Self :: from_iter_items (items . iter () . copied ()) } pub (crate) fn from_iter_items (iter_items : impl Iterator < Item = PatternItem >) -> Self { let time_granularity : TimeGranularity = iter_items . map (Into :: into) . max () . unwrap_or_default () ; Self :: from_time_granularity (time_granularity) } # [doc = " Merges the metadata from a date pattern and a time pattern into one."] # [inline] pub (crate) fn merge_date_and_time_metadata (_date : PatternMetadata , time : PatternMetadata ,) -> PatternMetadata { time } # [doc = " Creates a [`PatternMetadata`] from the [`TimeGranularity`] enum."] # [inline] pub const fn from_time_granularity (time_granularity : TimeGranularity) -> Self { Self (time_granularity . ordinal ()) } # [cfg (feature = "datagen")] # [inline] pub (crate) fn set_time_granularity (& mut self , time_granularity : TimeGranularity) { self . 0 = time_granularity . ordinal () ; } pub (crate) fn to_four_bit_metadata (self) -> FourBitMetadata { # [expect (clippy :: unwrap_used)] FourBitMetadata :: try_from_byte (self . 0) . unwrap () } pub (crate) fn from_u8 (other : u8) -> Self { Self (TimeGranularity :: from_ordinal (other) . ordinal ()) } }
};
}
