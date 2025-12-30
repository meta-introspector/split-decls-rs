// Generated macro for impl_432 (impl)
macro_rules! Depcrate_util_prefilterimpl_432 {
() => {
// Module: crate::util::prefilter
// Provides: {"impl_432"}
// Dependencies: {}
impl RareByteOffset { # [doc = " Create a new rare byte offset. If the given offset is too big, then"] # [doc = " None is returned. In that case, callers should render the rare bytes"] # [doc = " prefilter inert."] fn new (max : usize) -> Option < RareByteOffset > { if max > u8 :: MAX as usize { None } else { Some (RareByteOffset { max : max as u8 }) } } }
};
}
