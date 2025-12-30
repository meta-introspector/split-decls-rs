// Generated macro for impl_289 (impl)
macro_rules! Depcrate_pattern_namesimpl_289 {
() => {
// Module: crate::pattern::names
// Provides: {"impl_289"}
// Dependencies: {}
impl DateTimeNamesMetadata { # [doc = " No impl Default: emphasize when we create a new empty instance"] # [inline] pub (crate) fn new_empty () -> Self { Self { zone_checksum : None , } } # [doc = " If mz_periods is already populated, we can't load anything else because"] # [doc = " we can't verify the checksum. Set a blank checksum in this case."] # [inline] pub (crate) fn new_from_previous < M : DateTimeNamesMarker > (names : & RawDateTimeNames < M >) -> Self { if names . mz_periods . get () . inner . get_option () . is_some () { Self { zone_checksum : Some (0) , } } else { Self :: new_empty () } } }
};
}
