// Generated macro for impl_25 (impl)
macro_rules! Depcrateimpl_25 {
() => {
// Module: crate
// Provides: {"impl_25"}
// Dependencies: {}
# [cfg (windows)] impl < T > MmapAsRawDesc for & T where T : AsRawHandle , { fn as_raw_desc (& self) -> MmapRawDescriptor { MmapRawDescriptor (self . as_raw_handle ()) } }
};
}
