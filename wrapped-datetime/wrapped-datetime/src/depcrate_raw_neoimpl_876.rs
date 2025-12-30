// Generated macro for impl_876 (impl)
macro_rules! Depcrate_raw_neoimpl_876 {
() => {
// Module: crate::raw::neo
// Provides: {"impl_876"}
// Dependencies: {}
impl DateTimeInputUnchecked { fn resolve_time_precision (& self , time_precision : TimePrecision ,) -> (PackedSkeletonVariant , Option < SubsecondDigits >) { match time_precision { TimePrecision :: Hour => (PackedSkeletonVariant :: Standard , None) , TimePrecision :: Minute => (PackedSkeletonVariant :: Variant0 , None) , TimePrecision :: Second => (PackedSkeletonVariant :: Variant1 , None) , TimePrecision :: Subsecond (f) => (PackedSkeletonVariant :: Variant1 , Some (f)) , TimePrecision :: MinuteOptional => { let minute = self . minute . unwrap_or_default () ; if minute . is_zero () { (PackedSkeletonVariant :: Standard , None) } else { (PackedSkeletonVariant :: Variant0 , None) } } } } }
};
}
