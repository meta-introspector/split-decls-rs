// Generated macro for impl_70 (impl)
macro_rules! Depcrate_providerimpl_70 {
() => {
// Module: crate::provider
// Provides: {"impl_70"}
// Dependencies: {}
impl ListFormatterPatterns < '_ > { # [doc = " The marker attributes for narrow lists"] pub const NARROW : & 'static DataMarkerAttributes = DataMarkerAttributes :: from_str_or_panic ("N") ; # [doc (hidden)] pub const NARROW_STR : & 'static str = Self :: NARROW . as_str () ; # [doc = " The marker attributes for short lists"] pub const SHORT : & 'static DataMarkerAttributes = DataMarkerAttributes :: from_str_or_panic ("S") ; # [doc (hidden)] pub const SHORT_STR : & 'static str = Self :: SHORT . as_str () ; # [doc = " The marker attributes for wide lists"] pub const WIDE : & 'static DataMarkerAttributes = DataMarkerAttributes :: from_str_or_panic ("W") ; # [doc (hidden)] pub const WIDE_STR : & 'static str = Self :: WIDE . as_str () ; }
};
}
