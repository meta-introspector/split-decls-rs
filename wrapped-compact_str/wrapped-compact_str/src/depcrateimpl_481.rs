// Generated macro for impl_481 (impl)
macro_rules! Depcrateimpl_481 {
() => {
// Module: crate
// Provides: {"impl_481"}
// Dependencies: {}
# [cfg (target_has_atomic = "ptr")] impl From < CompactString > for alloc :: sync :: Arc < str > { fn from (value : CompactString) -> Self { Self :: from (value . as_str ()) } }
};
}
